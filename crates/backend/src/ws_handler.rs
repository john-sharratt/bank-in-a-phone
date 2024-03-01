use anyhow::Context;
use futures_util::StreamExt;
use immutable_bank_model::ledger_entry::LedgerEntry;
use std::net::{Ipv6Addr, SocketAddr};
use tokio_tungstenite::tungstenite::Message;

use tokio::net::{TcpListener, TcpStream};

use crate::{general_state::GeneralState, opts::Opts};

pub async fn ws_server(opts: &Opts, state: GeneralState) -> anyhow::Result<()> {
    // Create a TCP listener on the port
    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, opts.ws_port));
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Running HTTP server on: {:?}", addr);

    // Accept all the connections
    loop {
        let (stream, addr) = match listener.accept().await {
            Ok(a) => a,
            Err(err) => {
                tracing::error!("Failed to accept WS connection - {}", err);
                continue;
            }
        };
        tracing::info!("Accepted WS connection from: {:?}", addr);

        let state = state.clone();
        tokio::spawn(async move {
            if let Err(err) = handle_ws(stream, state).await {
                tracing::error!("WebSocket has failed: {}", err);
            }
        });
    }
}

async fn handle_ws(stream: TcpStream, state: GeneralState) -> anyhow::Result<()> {
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .context("Error during the websocket handshake occurred")?;

    let (tx, mut rx) = ws_stream.split();

    // Subscribe
    state.subscribe(tx);

    // Read all the messages
    loop {
        let msg = rx.next().await.context("Failed to read next message");
        match msg?? {
            Message::Text(txt) => {
                tracing::debug!("WS message text - {}", txt);
            }
            Message::Binary(data) => {
                let entry: LedgerEntry = bincode::deserialize(&data)?;

                let broadcast = {
                    let mut guard = state.lock().await;
                    let start = guard.ledger.entries.len();
                    guard.process(entry);
                    guard
                        .ledger
                        .entries
                        .iter()
                        .skip(start)
                        .cloned()
                        .collect::<Vec<_>>()
                };
                for broadcast in broadcast {
                    state.broadcast(broadcast);
                }
            }
            Message::Ping(_) => {
                tracing::debug!("WS message: PING");
            }
            Message::Pong(_) => {
                tracing::debug!("WS message: PONG");
            }
            Message::Close(_) => {
                tracing::debug!("Closing web socket");
                break;
            }
            Message::Frame(_) => {
                tracing::debug!("Unhandled WS message: frame");
            }
        }
    }

    Ok(())
}

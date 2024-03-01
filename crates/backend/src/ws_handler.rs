use anyhow::Context;
use futures_util::StreamExt;
use hyper_tungstenite::HyperWebsocket;
use immutable_bank_model::ledger_entry::LedgerEntry;
use tokio_tungstenite::tungstenite::Message;

use crate::general_state::GeneralState;

pub async fn handle_ws(stream: HyperWebsocket, state: GeneralState) -> anyhow::Result<()> {
    let ws_stream = stream
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

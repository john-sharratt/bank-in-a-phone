use anyhow::Context;
use futures_util::{SinkExt, StreamExt};
use hyper_tungstenite::HyperWebsocket;
use immutable_bank_model::header::{LedgerHeader, LedgerMessage};
use tokio_tungstenite::tungstenite::Message;

use crate::general_state::GeneralState;

pub async fn handle_ws(stream: HyperWebsocket, state: GeneralState) -> anyhow::Result<()> {
    let ws_stream = stream
        .await
        .context("Error during the websocket handshake occurred")?;

    let (mut tx, mut rx) = ws_stream.split();

    // Before we do anything we send the entire ledger to the client
    let entire_ledger = {
        let guard = state.inner.lock().await;
        guard
            .ledger
            .entries
            .iter()
            .map(|(a, b)| LedgerMessage {
                header: a.clone(),
                entry: b.clone(),
            })
            .collect::<Vec<_>>()
    };
    for msg in entire_ledger {
        tracing::warn!("Replay message: {:?}", msg);
        let data = bincode::serialize(&msg)?;
        tx.send(Message::binary(data)).await?;
    }

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
                let msg: LedgerMessage = bincode::deserialize(&data)?;

                let broadcast = {
                    let mut guard = state.lock().await;
                    let start = guard
                        .ledger
                        .entries
                        .last_key_value()
                        .map(|e| LedgerHeader {
                            id: e.0.id,
                            signature: 0,
                        })
                        .unwrap_or(LedgerHeader::ZERO);
                    guard.process(msg);
                    guard
                        .ledger
                        .entries
                        .range(start..)
                        .map(|a| LedgerMessage {
                            header: a.0.clone(),
                            entry: a.1.clone(),
                        })
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

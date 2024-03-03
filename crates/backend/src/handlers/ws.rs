use anyhow::Context;
use futures_util::{SinkExt, StreamExt};
use hyper_tungstenite::HyperWebsocket;
use immutable_bank_model::{bank_id::BankId, ledger::LedgerMessage};
use tokio_tungstenite::tungstenite::Message;

use crate::general_state::GeneralState;

pub async fn handle_ws<ID>(
    stream: HyperWebsocket,
    bank_id: ID,
    state: GeneralState,
) -> anyhow::Result<()>
where
    ID: Into<BankId>,
{
    let bank_id: BankId = bank_id.into();
    let ws_stream = stream
        .await
        .context("Error during the websocket handshake occurred")?;

    let (mut tx, mut rx) = ws_stream.split();

    // Before we do anything we send the entire ledger to the client
    let entire_ledger = {
        let guard = state.inner.lock().await;
        guard
            .ledger
            .entries_for(bank_id.clone())
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    };
    for msg in entire_ledger {
        tracing::debug!("Replay message: {:?}", msg);
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
                if msg.header.bank_id != bank_id {
                    continue;
                }

                if state.lock().await.process(&msg) {
                    state.broadcast(&msg);
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

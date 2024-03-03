use futures_util::{stream::SplitSink, SinkExt};
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use immutable_bank_model::ledger::LedgerMessage;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::general_state::GeneralState;

pub type SubscriberTx =
    SplitSink<WebSocketStream<TokioIo<Upgraded>>, tokio_tungstenite::tungstenite::Message>;

impl GeneralState {
    pub fn subscribe(&self, mut tx: SubscriberTx) {
        let mut rx = self.subscribers.subscribe();
        tokio::task::spawn(async move {
            while let Ok(data) = rx.recv().await {
                let res = tx.send(Message::binary(data.clone())).await;
                if let Err(err) = res {
                    tracing::debug!("WS send failed - {}", err);
                    return;
                }
            }
            tracing::error!("state subscription has closed");
        });
    }

    pub fn broadcast(&self, msg: &LedgerMessage) {
        let data = match bincode::serialize(msg) {
            Ok(d) => d,
            Err(err) => {
                tracing::error!("failed to serialize entry to broadcast - {}", err);
                return;
            }
        };

        if let Err(err) = self.subscribers.send(data.into()) {
            tracing::debug!("Broadcast failed - {}", err);
        }
    }
}

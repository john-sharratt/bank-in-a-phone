use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, sync::Arc, time::Duration};

use immutable_bank_model::{
    ledger::{Ledger, LedgerForBank, LedgerMessage},
    ledger_type::LedgerEntry,
};
use tokio::sync::{broadcast, Mutex, MutexGuard};

use crate::{opts::Opts, BROKER_SECRET};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeneralStateInner {
    pub ledger: Ledger,
}

#[derive(Debug, Clone)]
pub struct GeneralState {
    pub(crate) inner: Arc<Mutex<GeneralStateInner>>,
    pub(crate) subscribers: broadcast::Sender<Bytes>,
}

impl GeneralState {
    pub fn load(opts: &Opts) -> GeneralState {
        let msgs = match std::fs::read_to_string(&opts.data_path) {
            Ok(data) => {
                let msgs: Vec<LedgerMessage> = match serde_json::from_str(&data) {
                    Ok(inner) => inner,
                    Err(err) => {
                        tracing::error!(
                            "failed to deserialize the ledger - {:?} - {}",
                            opts.data_path,
                            err
                        );
                        Vec::new()
                    }
                };
                msgs
            }
            Err(err) => {
                tracing::error!("failed to read ledger - {:?} - {}", opts.data_path, err);
                Vec::new()
            }
        };

        let mut inner = GeneralStateInner::default();
        for msg in msgs {
            let ledger = match &msg.entry {
                LedgerEntry::NewBank { bank_secret, .. } => inner
                    .ledger
                    .banks
                    .entry(msg.header.bank_id.clone())
                    .or_insert_with(|| LedgerForBank {
                        broker_secret: BROKER_SECRET.clone(),
                        bank_secret: bank_secret.clone(),
                        entries: Default::default(),
                    }),
                _ => match inner.ledger.banks.get_mut(&msg.header.bank_id) {
                    Some(l) => l,
                    None => continue,
                },
            };
            ledger.entries.insert(msg.broker_signature.clone(), msg);
        }

        let state = GeneralState {
            inner: Arc::new(Mutex::new(inner)),
            subscribers: tokio::sync::broadcast::channel(10_000).0,
        };

        {
            let opts = opts.clone();
            let state = state.clone();
            tokio::task::spawn(async move {
                state.background_save(opts).await;
            });
        }

        state
    }

    pub async fn background_save(&self, opts: Opts) {
        let mut interval = tokio::time::interval(Duration::from_secs(opts.save_frequency));
        loop {
            interval.tick().await;

            // Copy the state
            let msgs = {
                let guard = self.lock().await;
                let entries = tokio::task::block_in_place(|| {
                    guard
                        .deref()
                        .ledger
                        .banks
                        .iter()
                        .flat_map(|b| b.1.entries.iter().map(|e| e.1))
                        .cloned()
                        .collect::<Vec<_>>()
                });
                tracing::info!(
                    "Saving general state to {:?} - entries.len={}",
                    opts.data_path,
                    entries.len()
                );
                entries
            };

            // Determine the staging location
            let mut staging_path = opts.data_path.clone();
            let mut filename = staging_path.file_name().clone().unwrap().to_owned();
            filename.push(".staging");
            staging_path.set_file_name(filename);

            // We are going into a blocking thread while we save the data to the disk
            // This operation is done in a safe way not to delete the journal
            tokio::task::block_in_place(|| {
                if let Ok(data) = serde_json::to_vec_pretty(&msgs) {
                    if std::path::Path::exists(&staging_path) {
                        if let Err(err) = std::fs::remove_file(&staging_path) {
                            tracing::error!("failed to remove staging file - {}", err);
                        }
                    }
                    if let Err(err) = std::fs::write(&staging_path, &data) {
                        tracing::error!("failed to write staging file - {}", err);
                    } else if let Err(err) = std::fs::rename(&staging_path, &opts.data_path) {
                        tracing::error!("failed to commit staging file - {}", err);
                    }
                }
            });
        }
    }

    pub async fn lock(&self) -> MutexGuard<'_, GeneralStateInner> {
        self.inner.lock().await
    }
}

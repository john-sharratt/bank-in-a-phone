use bytes::Bytes;
use serde::Serialize;

#[cfg(not(target_arch = "wasm32"))]
use tokio::task::spawn_local;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

use crate::{reqwest::post, LocalApp};

impl LocalApp {
    pub fn start_post<REQ, RES, F>(&mut self, path: &str, request: REQ, callback: F)
    where
        REQ: Serialize + std::fmt::Debug + 'static,
        RES: serde::de::DeserializeOwned + std::fmt::Debug + 'static,
        F: FnOnce(RES, &mut LocalApp, &mut eframe::Frame) + 'static,
    {
        log::info!("Sending request {:?}", request);

        let (tx, rx) = std::sync::mpsc::channel();

        let path = path.to_string();
        spawn_local(async move {
            let res = post(path, request).await;
            tx.send(res).ok();
        });

        let callback = move |data: Bytes, app: &mut LocalApp, frame: &mut eframe::Frame| {
            let res: RES = serde_json::from_slice(&data)?;
            Ok(callback(res, app, frame))
        };

        self.pending.replace(rx);
        self.pending_callback.replace(Box::new(callback));
    }
}

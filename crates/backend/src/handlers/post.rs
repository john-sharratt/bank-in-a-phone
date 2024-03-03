use bytes::Bytes;
use http::{Request, Response, StatusCode};
use http_body_util::{BodyExt, Full};

use crate::{general_state::GeneralState, BROKER_SECRET};

// Sending more than a megabyte is prevented
pub const MAX_REQUEST_BODY: usize = 1024 * 1024;

pub async fn post_handler(
    req: Request<hyper::body::Incoming>,
    state: GeneralState,
) -> anyhow::Result<Response<Full<Bytes>>> {
    let (parts, mut body) = req.into_parts();

    // Read all the data to a particular limit and then fail
    // (this is to prevent DDOS attacks)
    let mut data: Vec<u8> = Vec::with_capacity(4906);
    while let Some(frame) = body.frame().await {
        if let Some(frame) = frame?.data_ref() {
            if data.len() > frame.len() {
                return Ok(Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(Full::new(Bytes::from("DDOS protection")))?);
            }
            data.extend_from_slice(frame);
        }
    }

    // pattern match for both the method and the path of the request
    let res = match (parts.method, parts.uri.path()) {
        (hyper::Method::POST, "/update-bank") => {
            let req = serde_json::from_slice(&data)?;
            let state_inner = state.clone();
            let res = state
                .inner
                .lock()
                .await
                .ledger
                .update_bank(req, move |msg| state_inner.broadcast(msg))?;
            tracing::info!("UpdateBank-Response: {:?}", res);
            serde_json::to_vec_pretty(&res)?
        }
        (hyper::Method::POST, "/new-bank") => {
            let req = serde_json::from_slice(&data)?;
            let state_inner = state.clone();
            let res =
                state
                    .inner
                    .lock()
                    .await
                    .ledger
                    .new_bank(&BROKER_SECRET, req, move |msg| state_inner.broadcast(msg))?;
            tracing::info!("NewBank-Response: {:?}", res);
            serde_json::to_vec_pretty(&res)?
        }
        (hyper::Method::POST, "/copy-bank") => {
            let req = serde_json::from_slice(&data)?;
            let res = state.inner.lock().await.ledger.copy_bank(req)?;
            tracing::info!("CopyBank-Response: {:?}", res);
            serde_json::to_vec_pretty(&res)?
        }
        (hyper::Method::POST, "/transfer") => {
            let req = serde_json::from_slice(&data)?;
            let state_inner = state.clone();
            let res = state
                .inner
                .lock()
                .await
                .ledger
                .transfer(req, move |msg| state_inner.broadcast(msg))?;
            tracing::info!("Transfer-Response: {:?}", res);
            serde_json::to_vec_pretty(&res)?
        }
        // Anything else handler
        _ => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Full::new(Bytes::from("Invalid Request")))?)
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Full::new(Bytes::from(res)))?)
}

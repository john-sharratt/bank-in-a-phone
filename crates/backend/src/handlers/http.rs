use http::{Method, StatusCode};
use http_body_util::Full;
use hyper::{body::Bytes, Request, Response};
use immutable_bank_model::bank_id::BankId;

use crate::{
    general_state::GeneralState,
    handlers::{get::get_handler, options::options_handler, post::post_handler, ws::handle_ws},
};

pub async fn http_handler(
    mut req: Request<hyper::body::Incoming>,
    state: GeneralState,
) -> anyhow::Result<Response<Full<Bytes>>> {
    tracing::debug!("Request: method={}, url={}", req.method(), req.uri());

    if req.method() == &Method::OPTIONS {
        return options_handler(req).await;
    }

    if hyper_tungstenite::is_upgrade_request(&req) {
        tracing::debug!("Request: upgrading to websocket");
        let (response, websocket) = hyper_tungstenite::upgrade(&mut req, None)?;

        // Get the bank ID from the request query string
        let bank_path = req.uri().path().to_lowercase();
        let bank_id = match bank_path.split_once("/bank/") {
            Some((_, bank_id)) => BankId::from(bank_id),
            None => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Full::new(Bytes::from("Invalid Request")))?);
            }
        };

        // Spawn a task to handle the websocket connection.
        tokio::spawn(async move {
            if let Err(e) = handle_ws(websocket, bank_id, state).await {
                tracing::error!("Error in websocket connection: {e}");
            }
        });

        // Return the response so the spawned future can continue.
        return Ok(response);
    }

    // If its a GET request
    if req.method() == Method::GET {
        return get_handler(req).await;
    }

    // Maybe its a API call
    if req.method() == Method::POST {
        return post_handler(req, state).await;
    }

    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Full::new(Bytes::from("Invalid Request")))?)
}

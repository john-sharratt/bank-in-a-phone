use std::net::{Ipv6Addr, SocketAddr};

use http::StatusCode;
use http_body_util::Full;
use hyper::{body::Bytes, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use include_dir::{include_dir, Dir};
use tokio::net::TcpListener;

use crate::{general_state::GeneralState, opts::Opts, ws_handler::handle_ws};

const HTML_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../crates/web/dist");

pub async fn http_server(opts: &Opts, state: GeneralState) -> anyhow::Result<()> {
    // Create a TCP listener on the port
    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, opts.http_port));
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Running HTTP server on: {:?}", addr);

    // Accept all the connections
    loop {
        let (stream, addr) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tracing::info!("Accepted connection from: {:?}", addr);

        let state = state.clone();
        tokio::task::spawn(async move {
            let mut http = hyper::server::conn::http1::Builder::new();
            http.keep_alive(true);

            if let Err(err) = http
                .serve_connection(
                    io,
                    service_fn(|req: Request<_>| {
                        let state = state.clone();
                        async move { http_handler(req, state).await }
                    }),
                )
                .with_upgrades()
                .await
            {
                tracing::error!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn http_handler(
    mut req: Request<hyper::body::Incoming>,
    state: GeneralState,
) -> anyhow::Result<Response<Full<Bytes>>> {
    tracing::debug!("Request: method={}, url={}", req.method(), req.uri());

    if hyper_tungstenite::is_upgrade_request(&req) {
        tracing::debug!("Request: upgrading to websocket");
        let (response, websocket) = hyper_tungstenite::upgrade(&mut req, None)?;

        // Spawn a task to handle the websocket connection.
        tokio::spawn(async move {
            if let Err(e) = handle_ws(websocket, state).await {
                eprintln!("Error in websocket connection: {e}");
            }
        });

        // Return the response so the spawned future can continue.
        return Ok(response);
    }

    // Get the path to the thing we loading and sanitize it
    let mut path = req.uri().path();

    // Special case and strip slash
    if path.is_empty() || path == "/" {
        path = "/index.html";
    }
    if path.starts_with("/") {
        path = &path[1..];
    }

    // Sanitize
    if path.contains("..") || path.starts_with("/") || path.starts_with("~") {
        tracing::warn!("Access denied: path={}", path);
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Full::new(Bytes::from("Access denied")))?);
    }

    // Load the file
    let file = match HTML_DIR.get_file(path) {
        Some(file) => file,
        None => {
            tracing::debug!("Not found: path={}", path);
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from("File not found")))?);
        }
    };

    // Write the response
    let mut res = Response::new(Full::new(Bytes::from(file.contents())));

    let meme = mime_guess::from_path(file.path()).first_or_octet_stream();
    res.headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_str(&meme.to_string())?,
    );
    res.headers_mut().insert(
        http::header::CACHE_CONTROL,
        http::HeaderValue::from_str("Cache-Control: max-age=30, stale-while-revalidate=86400")?,
    );

    tracing::debug!("Response: status={}, url={}", res.status(), req.uri());
    Ok(res)
}

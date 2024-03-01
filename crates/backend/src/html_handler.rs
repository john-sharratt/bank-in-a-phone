use std::net::{Ipv6Addr, SocketAddr};

use http::StatusCode;
use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use include_dir::{include_dir, Dir};
use tokio::net::TcpListener;

use crate::opts::Opts;

const HTML_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../crates/web/dist");

pub async fn http_server(opts: &Opts) -> anyhow::Result<()> {
    // Create a TCP listener on the port
    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, opts.http_port));
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Running HTTP server on: {:?}", addr);

    // Accept all the connections
    loop {
        let (stream, addr) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tracing::info!("Accepted connection from: {:?}", addr);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(http_handler))
                .await
            {
                tracing::error!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn http_handler(
    req: Request<hyper::body::Incoming>,
) -> anyhow::Result<Response<Full<Bytes>>> {
    tracing::debug!("Request: method={}, url={}", req.method(), req.uri());

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
    let data = match HTML_DIR.get_file(path) {
        Some(file) => file,
        None => {
            tracing::debug!("Not found: path={}", path);
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from("File not found")))?);
        }
    };

    // Write the response
    let res = Response::new(Full::new(Bytes::from(data.contents())));
    tracing::debug!("Response: status={}, url={}", res.status(), req.uri());
    Ok(res)
}

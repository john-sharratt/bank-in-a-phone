use std::net::{Ipv6Addr, SocketAddr};

use hyper::{service::service_fn, Request};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::{general_state::GeneralState, handlers::http::http_handler, opts::Opts};

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

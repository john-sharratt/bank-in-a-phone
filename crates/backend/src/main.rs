pub mod broadcast;
pub mod general_state;
pub mod http_handler;
pub mod logger;
pub mod opts;
pub mod process;
pub mod ws_handler;

use clap::Parser;
use opts::Opts;

use crate::{general_state::GeneralState, http_handler::http_server};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    // Attach the logging
    if let Some(log_level) = opts.verbosity.log_level() {
        logger::init(log_level)?;
    }

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(32)
        .max_blocking_threads(256)
        .thread_name("immutable-bank")
        .enable_all()
        .build()?
        .block_on(async {
            tracing::trace!("tokio runtime initialized");

            // Load the general state
            let state = GeneralState::load(&opts);

            // Spinning up the HTTP server
            {
                let opts = opts.clone();
                tokio::task::spawn(async move {
                    http_server(&opts, state).await.unwrap();
                });
            }

            // Waiting for a ctrl-c
            tokio::signal::ctrl_c().await?;
            tracing::info!("ctrl-c()::exiting...");
            Ok::<(), anyhow::Error>(())
        })?;

    /*
    // Start listening for http connections
    thread::spawn(|| {
        let http_server = HttpServer::http("127.0.0.1:8000").unwrap();
        http_server.handle(http_handler).unwrap();
    });

    // Start listening for WebSocket connections
    let ws_server = Server::bind("127.0.0.1:8001").unwrap();
    for connection in ws_server.filter_map(Result::ok) {
        // Spawn a new thread for each connection.
        thread::spawn(|| {
            if !connection
                .protocols()
                .contains(&"rust-websocket".to_string())
            {
                connection.reject().unwrap();
                return;
            }

            let mut client = connection.use_protocol("rust-websocket").accept().unwrap();

            let ip = client.peer_addr().unwrap();

            println!("Connection from {}", ip);

            let message = Message::text("Hello");
            client.send_message(&message).unwrap();

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Close(_) => {
                        let message = Message::close();
                        sender.send_message(&message).unwrap();
                        println!("Client {} disconnected", ip);
                        return;
                    }
                    OwnedMessage::Ping(data) => {
                        let message = Message::pong(data);
                        sender.send_message(&message).unwrap();
                    }
                    _ => sender.send_message(&message).unwrap(),
                }
            }
        });
    }
    */

    Ok(())
}

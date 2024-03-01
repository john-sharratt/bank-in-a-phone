#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub mod render;
pub mod state;
pub mod ws;
pub mod sleep;
pub mod process;
pub use state::local_app::LocalApp;

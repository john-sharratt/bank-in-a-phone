#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub mod render;
pub mod state;
pub use state::local_app::LocalApp;

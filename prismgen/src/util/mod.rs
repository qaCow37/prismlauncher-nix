pub mod client;
pub mod prism;
pub mod nix;

pub use client::Client;
pub use nix::Nix;
pub type PrismResponse = prism::Response;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

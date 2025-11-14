mod client;
pub mod api;
pub mod auth;
pub mod models;
pub mod licensing;

pub const XBOX_LIVE_PACKAGES_PC: &str = "https://packagespc.xboxlive.com";

pub use xal;
pub use reqwest;
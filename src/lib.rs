#![warn(rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]
pub mod client;
mod error;
pub mod schema;
pub mod users;
pub use client::GithubClient;
#[cfg(feature = "auth")]
pub use client::GithubClientBuilder;
#[cfg(feature = "auth")]
pub use error::creation::CreationError;
pub use error::runtime::RuntimeError;
#[cfg(test)]
pub(crate) mod constants;
#[cfg(test)]
pub(crate) mod test_utils;
pub(crate) mod utils;

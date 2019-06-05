mod client;
mod gesetz;
mod toc;

pub use toc::{Toc, TocItem};
pub use gesetz::Gesetz;
pub use client::Client;

#[cfg(test)]
mod tests;

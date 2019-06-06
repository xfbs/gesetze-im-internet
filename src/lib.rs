mod client;
mod gesetz;
mod toc;

pub use client::Client;
pub use gesetz::Gesetz;
pub use toc::{Toc, TocItem};

#[cfg(test)]
mod tests;

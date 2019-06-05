#[cfg(test)]
mod tests;

mod http_client;
mod gesetz;
mod toc;

pub use toc::{Toc, TocItem};
pub use gesetz::Gesetz;

pub struct Gesetze {
}


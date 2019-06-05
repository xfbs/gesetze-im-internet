mod client;
mod gesetz;
mod toc;

pub use toc::{Toc, TocItem};
pub use gesetz::Gesetz;

#[cfg(test)]
mod tests;

mod client;
mod gesetz;
mod toc;

pub use gesetz::Gesetz;
pub use toc::{Toc, TocItem};

#[cfg(test)]
mod tests;

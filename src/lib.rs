#[cfg(test)]
mod tests;

mod gesetz;
mod toc;

pub use toc::{Toc, TocItem};
pub use gesetz::Gesetz;

pub struct Gesetze {
}


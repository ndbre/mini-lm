pub type Id = usize;
pub type Token = String;

const UNKNOWN_ID: Id = usize::MAX;

pub mod bpe;
pub mod vocab;

pub use bpe::*;
pub use vocab::*;
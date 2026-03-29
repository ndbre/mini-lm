pub mod progress;
pub use progress::*;

use crate::ngram::NgramModel;
use crate::tokenizer::BPETokenizer;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

use postcard;

#[derive(Serialize, Deserialize)]
struct SavedModel {
    tokenizer: BPETokenizer,
    model: NgramModel,
}

pub fn save(path: &str, tokenizer: &BPETokenizer, model: &NgramModel) -> io::Result<()> {
    let saved = SavedModel {
        tokenizer: tokenizer.clone(),
        model: model.clone(),
    };
    let bytes =
        postcard::to_allocvec(&saved).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(path, bytes)
}

pub fn load(path: &str) -> io::Result<(BPETokenizer, NgramModel)> {
    let bytes = fs::read(path)?;
    let saved: SavedModel =
        postcard::from_bytes(&bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok((saved.tokenizer, saved.model))
}

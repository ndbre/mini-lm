use super::vocab::Vocab;
use super::{Id, UNKNOWN_ID};
use crate::io::{ProgressBar, ProgressBarStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BPETokenizer {
    pub vocab: Vocab,
    pub merges: Vec<(Id, Id, Id)>,
}

impl BPETokenizer {
    pub fn train(text: &str, num_merges: usize) -> BPETokenizer {
        let mut vocab = Vocab::new();

        for ch in text.chars() {
            vocab.add_token(ch.to_string());
        }

        let mut tokens: Vec<Id> = text
            .chars()
            .map(|ch| vocab.get_id(&ch.to_string()).unwrap())
            .collect();

        let mut merges: Vec<(Id, Id, Id)> = Vec::new();

        let bar = ProgressBar::new(num_merges as u64, ProgressBarStyle::default());

        for merge_idx in 0..num_merges {
            if tokens.len() < 2 {
                break;
            }

            let mut pair_counts: HashMap<(Id, Id), usize> = HashMap::new();
            for i in 0..tokens.len() - 1 {
                let pair = (tokens[i], tokens[i + 1]);
                *pair_counts.entry(pair).or_insert(0) += 1;
            }

            if pair_counts.is_empty() {
                break;
            }

            let best_pair = pair_counts
                .iter()
                .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0)))
                .map(|(&pair, _)| pair)
                .unwrap();

            let best_count = pair_counts[&best_pair];
            if best_count < 2 {
                println!(
                    "Stopped merging early at {} merges, no pairs appear more than oncee.",
                    merge_idx
                );
                break;
            }

            let left_str = vocab.get_token(best_pair.0).unwrap().to_string();
            let right_str = vocab.get_token(best_pair.1).unwrap().to_string();
            let new_str = format!("{}{}", left_str, right_str);

            let new_id = vocab.add_token(new_str);
            merges.push((best_pair.0, best_pair.1, new_id));

            tokens = apply_merge(&tokens, best_pair.0, best_pair.1, new_id);

            if (merge_idx + 1) % 10 == 0 || merge_idx == 0 {
                bar.update(merge_idx as u64 + 1);
            }
        }
        bar.finish(format!(
            "Finished Training BPE Tokenizer with a Vocab size of {} after {} Merges.",
            vocab.size(),
            merges.len()
        ));

        BPETokenizer {
            vocab: vocab,
            merges: merges,
        }
    }

    pub fn encode(&self, text: &str) -> Vec<usize> {
        let mut tokens: Vec<usize> = text
            .chars()
            .map(|ch| self.vocab.get_id(&ch.to_string()).unwrap_or(UNKNOWN_ID))
            .collect();

        for &(left, right, merged) in &self.merges {
            tokens = apply_merge(&tokens, left, right, merged);
        }

        tokens
    }

    pub fn decode(&self, token_ids: &[usize]) -> String {
        let mut result = String::new();
        for &id in token_ids {
            match self.vocab.get_token(id) {
                Some(s) => result.push_str(s),
                None => result.push_str("<unk>"),
            }
        }
        result
    }
}

fn apply_merge(tokens: &[Id], left: Id, right: Id, merged: Id) -> Vec<Id> {
    let mut result = Vec::with_capacity(tokens.len());

    let mut i = 0;
    while i < tokens.len() {
        if i + 1 < tokens.len() && tokens[i] == left && tokens[i + 1] == right {
            result.push(merged);
            i += 2;
        } else {
            result.push(tokens[i]);
            i += 1;
        }
    }
    result
}

use super::Id;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgramModel {
    pub n: usize,
    pub table: HashMap<Vec<Id>, HashMap<Id, usize>>,
}

impl NgramModel {
    pub fn train(tokens: &[Id], n: usize) -> NgramModel {
        assert!(n >= 2, "N must be at least 2.");
        assert!(
            tokens.len() >= n,
            "Number of tokens must be more than N. (Training Data too small for N)"
        );

        let mut table: HashMap<Vec<Id>, HashMap<Id, usize>> = HashMap::new();

        for order in 2..=n {
            let ctx_len = order - 1;
            for i in 0..=tokens.len() - order {
                let context = tokens[i..i + ctx_len].to_vec();
                let next = tokens[i + ctx_len];

                *table
                    .entry(context)
                    .or_insert_with(HashMap::new)
                    .entry(next)
                    .or_insert(0) += 1;
            }
        }

        println!(
            "N-gram training complete. n = {}. unique contexts: {}",
            n,
            table.len()
        );

        NgramModel { n, table }
    }

    pub fn n_contexts(&self) -> usize {
        self.table.len()
    }

    pub fn generate(&self, seed: &[Id], num_tokens: usize) -> Vec<Id> {
        let context_len = self.n - 1;

        let mut output: Vec<Id> = seed.to_vec();
        assert!(!output.is_empty(), "There must be a seed phrase.");

        while output.len() < context_len {
            let first = output[0];
            output.insert(0, first); // Pad begining with first character to fit context len.
        }

        let mut backoff_count = 0usize;

        for _ in 0..num_tokens {
            let mut next_token = None;
            for backoff in 0..context_len {
                let ctx_start = output.len() - (context_len - backoff);
                let ctx = output[ctx_start..].to_vec();
                if let Some(counts) = self.table.get(&ctx) {
                    next_token = Some(weighted_sample(counts));
                    if backoff > 0 {
                        backoff_count += 1;
                    }
                    break;
                }
            }

            match next_token {
                None => break,
                Some(t) => output.push(t),
            }
        }

        if backoff_count > 0 {
            println!(
                "Backed off to shorter context {} time(s) while generating.",
                backoff_count
            );
        }

        output
    }
}

fn weighted_sample(counts: &HashMap<Id, usize>) -> Id {
    let total: usize = counts.values().sum();
    let threshold = rand::random_range(0..total);

    let mut cumulative = 0usize;
    for (&token_id, &count) in counts.iter() {
        cumulative += count;
        if cumulative > threshold {
            return token_id;
        }
    }

    *counts.keys().next().unwrap()
}

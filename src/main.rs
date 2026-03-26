type Id = usize;
type Token = String;

const UNKNOWN_ID: Id = usize::MAX;

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Vocab {
    token_to_id: HashMap<Token, Id>,
    id_to_token: HashMap<Id, Token>,
    next_id: Id,
}

impl Vocab {
    fn new() -> Vocab {
        Vocab {
            token_to_id: HashMap::new(),
            id_to_token: HashMap::new(),
            next_id: 0,
        }
    }
    fn add_token(&mut self, token: Token) -> Id {
        // Try to retrieve id of token. If its not there, then make a new one.
        if let Some(&id) = self.token_to_id.get(&token) {
            return id;
        }

        let id = self.next_id;
        self.token_to_id.insert(token.clone(), id);
        self.id_to_token.insert(id, token);
        self.next_id += 1;
        id
    }
    pub fn get_id(&self, token: &str) -> Option<Id> {
        self.token_to_id.get(token).copied()
    }

    pub fn get_token(&self, id: usize) -> Option<&str> {
        self.id_to_token.get(&id).map(|s| s.as_str())
    }

    pub fn size(&self) -> usize {
        self.token_to_id.len()
    }
}

#[derive(Debug, Clone)]
struct BPETokenizer {
    vocab: Vocab,
    merges: Vec<(Id, Id, Id)>,
}

impl BPETokenizer {
    fn train(text: &str, num_merges: usize) -> BPETokenizer {
        let mut vocab = Vocab::new();

        for ch in text.chars() {
            vocab.add_token(ch.to_string());
        }

        let mut tokens: Vec<Id> = text
            .chars()
            .map(|ch| vocab.get_id(&ch.to_string()).unwrap())
            .collect();

        let mut merges: Vec<(Id, Id, Id)> = Vec::new();

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
        }

        println!(
            "Finished Training BPE Tokenizer with a Vocab size of {} after {} Merges.",
            vocab.size(),
            merges.len()
        );

        BPETokenizer {
            vocab: vocab,
            merges: merges,
        }
    }

    fn encode(&self, text: &str) -> Vec<usize> {
        let mut tokens: Vec<usize> = text
            .chars()
            .map(|ch| {
                self.vocab
                    .get_id(&ch.to_string())
                    .unwrap_or(UNKNOWN_ID)
            })
            .collect();

        for &(left, right, merged) in &self.merges {
            tokens = apply_merge(&tokens, left, right, merged);
        }

        tokens
    }

    fn decode(&self, token_ids: &[usize]) -> String {
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

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();

    let tokenizer = BPETokenizer::train(&input, 500);

    println!("{:#?}", tokenizer);

    let encoded = tokenizer.encode("Hello, World.");

    println!("Encoding 'Hello, World.': {:?}", encoded);
    println!("Decoded: {:?}", tokenizer.decode(&encoded));
}

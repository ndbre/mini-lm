type Id = usize;
type Token = String;

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
            return id
        }

        let id = self.next_id;
        self.token_to_id.insert(token.clone(), id);
        self.id_to_token.insert(id, token);
        self.next_id += 1; 
        id
    }
    pub fn get_id(&self, token: &str) -> Option<usize> {
        self.token_to_id.get(token).copied()
    }

    pub fn get_token(&self, id: usize) -> Option<&str> {
        self.id_to_token.get(&id).map(|s| s.as_str())
    }

    pub fn size(&self) -> usize {
        self.token_to_id.len()
    } 
}

fn main() {
    let mut vocab = Vocab::new();

    vocab.add_token("a".to_string());
    vocab.add_token("1".to_string());
    vocab.add_token(".".to_string());
    vocab.add_token("'".to_string());
    vocab.add_token("`".to_string());
    vocab.add_token("ab".to_string());
    vocab.add_token("\t".to_string());

    println!("{:?}", vocab);
}

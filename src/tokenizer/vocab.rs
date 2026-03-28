use std::collections::HashMap;
use super::{UNKNOWN_ID, Id, Token};

#[derive(Debug, Clone)]
pub struct Vocab {
    token_to_id: HashMap<Token, Id>,
    id_to_token: HashMap<Id, Token>,
    next_id: Id,
}

impl Vocab {
    pub fn new() -> Vocab {
        Vocab {
            token_to_id: HashMap::new(),
            id_to_token: HashMap::new(),
            next_id: 0,
        }
    }
    pub fn add_token(&mut self, token: Token) -> Id {
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
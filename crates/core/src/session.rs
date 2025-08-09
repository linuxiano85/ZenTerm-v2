//! Session management

use std::collections::HashMap;

/// Represents a ZenTerm session
pub struct Session {
    pub id: String,
    pub context: HashMap<String, String>,
}

impl Session {
    pub fn new(id: String) -> Self {
        Self {
            id,
            context: HashMap::new(),
        }
    }
}
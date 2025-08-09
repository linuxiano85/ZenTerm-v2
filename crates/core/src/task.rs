//! Task definitions and graph

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a task in the execution graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub dependencies: Vec<Uuid>,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            dependencies: Vec::new(),
        }
    }
}
use std::any::Any;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum EventPayload {
    Text(String),
    Json(serde_json::Value),
    Binary(Vec<u8>),
    Custom(Arc<dyn Any + Send + Sync>),
}

impl EventPayload {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            EventPayload::Text(s) => Some(s),
            _ => None,
        }
    }
}
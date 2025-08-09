use std::time::SystemTime;

use super::key::EventKey;
use super::payload::EventPayload;

#[derive(Debug)]
pub struct Event {
    pub key: EventKey,
    pub payload: EventPayload,
    pub timestamp: SystemTime,
}

impl Event {
    pub fn new(key: EventKey, payload: EventPayload) -> Self {
        Self { key, payload, timestamp: SystemTime::now() }
    }
}
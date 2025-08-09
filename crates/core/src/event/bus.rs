use std::future::Future;
use std::pin::Pin;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use dashmap::DashMap;
use parking_lot::RwLock;
use tracing::debug;

use super::event::Event;
use super::key::{EventKey, EventPattern};
use super::payload::EventPayload;
use super::subscription::Subscription;

pub type HandlerFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
pub type HandlerFn = dyn Fn(Arc<Event>) -> HandlerFuture + Send + Sync + 'static;

pub struct HandlerEntry {
    pub(crate) id: u64,
    pub(crate) pattern: EventPattern,
    pub(crate) func: Arc<HandlerFn>,
}

pub struct InnerBus {
    partitions: DashMap<String, Vec<HandlerEntry>>, // keyed by first segment
    wildcard: RwLock<Vec<HandlerEntry>>,            // patterns starting with * or **
    seq: AtomicU64,
}

#[derive(Clone)]
pub struct EventBus {
    inner: Arc<InnerBus>,
}

impl Default for EventBus { fn default() -> Self { Self::new() } }

impl EventBus {
    pub fn new() -> Self {
        Self { inner: Arc::new(InnerBus { partitions: DashMap::new(), wildcard: RwLock::new(Vec::new()), seq: AtomicU64::new(1) }) }
    }

    pub fn subscribe<F, Fut>(&self, pattern: &str, handler: F) -> Subscription
    where
        F: Fn(Arc<Event>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let id = self.inner.seq.fetch_add(1, Ordering::Relaxed);
        let pat = EventPattern::parse(pattern);
        let func: Arc<HandlerFn> = Arc::new(move |evt| Box::pin(handler(evt)));
        let entry = HandlerEntry { id, pattern: pat.clone(), func };

        let first_segment = pat.first_segment().unwrap_or("");
        let is_wild = matches!(first_segment, "*" | "**");
        if is_wild {
            self.inner.wildcard.write().push(entry);
        } else {
            self.inner.partitions.entry(first_segment.to_string()).or_default().push(entry);
        }

        Subscription { id, inner: Arc::clone(&self.inner) }
    }

    pub fn emit(&self, key: impl AsRef<str>, payload: EventPayload) {
        let bus = self.clone();
        let key_str = key.as_ref().to_string();
        tokio::spawn(async move { let _ = bus.emit_wait(key_str, payload).await; });
    }

    pub async fn emit_wait(&self, key: impl AsRef<str>, payload: EventPayload) -> usize {
        let key = EventKey::parse(key.as_ref());
        let evt = Arc::new(Event::new(key.clone(), payload));
        debug!(event = %key, "emit");
        let handlers = self.inner.collect_handlers(&key);
        let count = handlers.len();
        for h in handlers {
            let evt_clone = evt.clone();
            tokio::spawn(async move { (h.func)(evt_clone).await; });
        }
        count
    }
}

impl InnerBus {
    fn collect_handlers(&self, key: &EventKey) -> Vec<HandlerEntry> {
        let mut out = Vec::new();
        if let Some(first) = key.segments().get(0) {
            if let Some(vec) = self.partitions.get(first) {
                for h in vec.iter() {
                    if h.pattern.matches(key) {
                        out.push(HandlerEntry { id: h.id, pattern: h.pattern.clone(), func: h.func.clone() });
                    }
                }
            }
        }
        for h in self.wildcard.read().iter() {
            if h.pattern.matches(key) {
                out.push(HandlerEntry { id: h.id, pattern: h.pattern.clone(), func: h.func.clone() });
            }
        }
        out
    }

    pub(crate) fn unsubscribe(&self, id: u64) {
        for mut bucket in self.partitions.iter_mut() {
            bucket.retain(|h| h.id != id);
        }
        self.wildcard.write().retain(|h| h.id != id);
    }
}
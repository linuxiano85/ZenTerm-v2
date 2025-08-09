use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use zenterm_core::event::prelude::*;

#[tokio::test]
async fn basic_subscribe_and_emit() {
    let bus = EventBus::new();
    let hits = Arc::new(AtomicUsize::new(0));

    {
        let hits = hits.clone();
        let _sub = bus.subscribe("voice.command.*", move |evt| {
            let hits = hits.clone();
            async move {
                if let Some(t) = evt.payload.as_text() { assert!(t.contains("run")); }
                hits.fetch_add(1, Ordering::Relaxed);
            }
        });

        bus.emit("voice.command.run", EventPayload::Text("run test".into()));
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    assert_eq!(hits.load(Ordering::Relaxed), 1);
}

#[tokio::test]
async fn unsubscribe_on_drop() {
    let bus = EventBus::new();
    let hits = Arc::new(AtomicUsize::new(0));
    {
        let hits = hits.clone();
        let _sub = bus.subscribe("core.*", move |_evt| {
            let hits = hits.clone();
            async move { hits.fetch_add(1, Ordering::Relaxed); }
        });
        bus.emit("core.start", EventPayload::Text("x".into()));
        tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    }
    bus.emit("core.start", EventPayload::Text("y".into()));
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    assert_eq!(hits.load(Ordering::Relaxed), 1);
}

#[tokio::test]
async fn pattern_double_star_tail() {
    let bus = EventBus::new();
    let hits = Arc::new(AtomicUsize::new(0));
    let hits2 = hits.clone();
    let _sub = bus.subscribe("plugin.**", move |_evt| {
        let hits = hits2.clone();
        async move { hits.fetch_add(1, Ordering::Relaxed); }
    });
    bus.emit("plugin", EventPayload::Text("a".into()));
    bus.emit("plugin.git", EventPayload::Text("b".into()));
    bus.emit("plugin.git.push", EventPayload::Text("c".into()));
    tokio::time::sleep(std::time::Duration::from_millis(60)).await;
    assert_eq!(hits.load(Ordering::Relaxed), 3);
}
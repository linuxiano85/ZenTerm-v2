use zenterm_core::event::prelude::*;

#[tokio::main]
async fn main() {
    let bus = EventBus::new();

    let _sub = bus.subscribe("voice.command.*", |_evt| async move {
        println!("[handler] voice command received");
    });

    let _sub2 = bus.subscribe("plugin.**", |evt| async move {
        println!("[handler] plugin event: {}", evt.key);
    });

    bus.emit("voice.command.run", EventPayload::Text("run ls".into()));
    bus.emit("plugin.git.sync", EventPayload::Text("sync upstream".into()));

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}
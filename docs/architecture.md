# Architecture

## Goals
1. Composability (plugins & providers)
2. Deterministic core with concurrent task execution
3. Extensibility: AI models, protocol bridges, automation actions
4. Observability built‑in (tracing + metrics optional)

## Layers
- CLI (argument parsing, UX)
- Core Engine (session, task graph, execution planner, state cache)
- Plugin Runtime (registry, lifecycle, extension points)
- AI Providers (abstraction + concrete adapters)
- Plugins (domain features: git, fs, process, shell, etc.)

## Data Flow (Simplified)
```
User -> CLI -> Core Command Router -> Task Graph Builder -> Executor (Tokio) ->
  (a) Plugin Calls
  (b) AI Provider Calls
  (c) Resource Adapters
 -> Result Aggregator -> Output Formatter -> User
```

## Plugin Model
```rust
pub trait ZenPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str { "" }
    fn init(&self, ctx: &PluginContext) -> anyhow::Result<()> { Ok(()) }
}
```
Static registry + optional dynamic loading (feature = "dynamic").

## Task Graph
Tasks are units with:
- id (UUID)
- inputs/outputs
- execution fn (async)
- dependencies

Execution is a topologically scheduled concurrent pipeline with bounded parallelism.

## Telemetry
Enabled via feature flags:
- metrics (Prometheus)
- otlp (OTel export)

## Future
- WASM sandbox for untrusted plugins
- Multi‑session orchestration
- Remote execution backend
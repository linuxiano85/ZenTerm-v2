# ZenTerm v2

Next‑generation modular terminal automation & AI orchestration platform.

## Crates Overview
- core: Engine (sessions, task graph, resource management, plugin runtime)
- plugins-api: Stable API surface for dynamic / static plugins
- plugins/git_helper: Example built‑in plugin (Git utilities)
- ai-providers: Abstractions + providers (OpenAI, Anthropic, Ollama, etc.)
- cli: User facing binary `zenterm`

## High Level Architecture
See docs/architecture.md

## Quick Start
```bash
cargo run -p cli -- --help
```

## Configuration
See `examples/config.example.yaml`.

## Roadmap (initial)
- [ ] Dynamic plugin loading (dlopen) behind feature flag
- [ ] WASM sandbox for untrusted plugins
- [ ] Multi‑model AI provider routing & fallback
- [ ] Session persistence & replay
- [ ] Telemetry & metrics (Prometheus + OTLP optional)

## License
MIT

---
Generated initial scaffold.
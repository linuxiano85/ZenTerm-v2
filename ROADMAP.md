# ZenTerm v2 Roadmap

This roadmap outlines the planned development milestones for ZenTerm v2, organized by functional areas and development phases.

## Phase 1: Foundation & Core Features
*Target: Q1 2025*

### Voice Input MVP
- [x] CLI skeleton with voice subcommands
- [ ] Whisper tiny integration for offline STT
- [ ] Push-to-talk mechanism (configurable hotkey)
- [ ] Basic audio input handling and validation
- [ ] Voice command cancellation ("cancel", "annulla")
- [ ] Audio level testing and microphone configuration

### Intent Router Baseline
- [x] Plugin API traits (IntentHandler)
- [ ] Regex-based intent matching
- [ ] Intent scoring and conflict resolution
- [ ] Fallback to small LLM for ambiguous commands
- [ ] Context-aware intent processing
- [ ] Command history and learning

### Core Infrastructure
- [x] Workspace structure and crate organization
- [x] Structured logging with tracing
- [x] Plugin API foundational traits
- [ ] Configuration layering (default + user + workspace)
- [ ] Error handling and user feedback
- [ ] Basic session management

## Phase 2: Security & Elevation
*Target: Q2 2025*

### Elevation Manager
- [x] CLI elevation subcommands
- [ ] PAM integration for authentication
- [ ] Time-boxed elevated sessions (configurable timeout)
- [ ] Secure password prompt (no echo, no logging)
- [ ] Session activity monitoring
- [ ] Automatic session revocation on inactivity

### Audit & Policy Engine
- [ ] Audit log implementation with hash chaining
- [ ] YAML/TOML policy configuration
- [ ] Plugin permission management
- [ ] Action pattern matching for policy enforcement
- [ ] Double confirmation for high-risk operations
- [ ] Audit log viewer and export

## Phase 3: Theming Engine
*Target: Q2-Q3 2025*

### Palette Extraction & Generation
- [x] CLI theme subcommands
- [ ] Image-based palette extraction (k-means clustering)
- [ ] Prompt-based color generation (AI integration)
- [ ] WCAG contrast validation and adjustment
- [ ] Semantic color slot mapping
- [ ] Color harmony and accessibility checks

### Desktop Environment Renderers
- [ ] GNOME renderer (GTK CSS, gsettings schemas)
- [ ] KDE renderer (.colors files, Kvantum themes)
- [ ] Terminal renderer (Alacritty, GNOME Terminal profiles)
- [ ] Theme preview functionality (TUI)
- [ ] Theme export and packaging
- [ ] Wallpaper generation pipeline

## Phase 4: Plugin Ecosystem
*Target: Q3 2025*

### Plugin Capability Registry
- [x] Basic plugin trait definitions
- [x] Git helper plugin example
- [ ] Plugin discovery and registration
- [ ] Capability introspection and metadata
- [ ] Plugin lifecycle management
- [ ] Version compatibility checking
- [ ] Plugin dependency resolution

### Core Plugins
- [ ] File system operations plugin
- [ ] Process management plugin
- [ ] Network utilities plugin
- [ ] System information plugin
- [ ] Package manager integration
- [ ] Development tools integration

### Dynamic Loading (Optional)
- [ ] WASM sandbox for untrusted plugins
- [ ] Dynamic library loading (libloading)
- [ ] Plugin hot-reloading
- [ ] Security boundaries and isolation

## Phase 5: Advanced Features
*Target: Q4 2025*

### Enhanced AI Integration
- [x] AI provider abstraction layer
- [ ] Multiple LLM provider support (OpenAI, local models)
- [ ] Text-to-speech output (optional)
- [ ] Context-aware command suggestions
- [ ] Natural language to command translation
- [ ] Voice command refinement and learning

### Telemetry & Observability
- [ ] OpenTelemetry integration (feature-flagged)
- [ ] Prometheus metrics export
- [ ] Performance monitoring and profiling
- [ ] Usage analytics (privacy-respecting)
- [ ] Error reporting and diagnostics

### Advanced Configuration
- [ ] Profile-based configuration
- [ ] Environment-specific settings
- [ ] Plugin configuration validation
- [ ] Configuration migration tools
- [ ] Remote configuration sync (optional)

## Phase 6: Platform Expansion
*Target: 2026*

### Windows Support
- [ ] Windows theming integration (accent colors, terminal schemes)
- [ ] PowerShell profile generation
- [ ] Windows-specific plugin adaptations
- [ ] UAC integration for elevation
- [ ] Windows Terminal configuration

### Multi-Session & Remote
- [ ] Multi-session orchestration
- [ ] Remote execution backend
- [ ] Session sharing and collaboration
- [ ] Cloud integration options

## Continuous Improvements

### Developer Experience
- [x] Comprehensive issue and PR templates
- [x] Dual licensing (MIT OR BSD-2-Clause)
- [ ] Contributing guidelines and documentation
- [ ] Plugin development SDK
- [ ] API documentation and examples
- [ ] Integration testing framework

### Performance & Reliability
- [ ] Benchmark suite with criterion
- [ ] Property-based testing with proptest
- [ ] Memory usage optimization
- [ ] Startup time optimization
- [ ] Error recovery and resilience

### Documentation
- [ ] User guide and tutorials
- [ ] Plugin development guide
- [ ] Architecture deep-dive
- [ ] Security best practices
- [ ] Troubleshooting guide

## Status Legend
- [x] Completed
- [ ] Planned/In Progress
- [!] Blocked/Needs Decision
- [~] Partially Complete

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for information on how to contribute to ZenTerm development.

Last updated: 2025-08-09
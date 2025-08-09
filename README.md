# ZenTerm v2

Semplicità d'uso + Potenza Estendibile.

ZenTerm v2 è un orchestratore/assistente da terminale con obiettivo doppio:
1. Ridurre la frizione quotidiana (voce, comandi naturali, automazione contestuale).
2. Espandere profondamente le capacità operative sul sistema (anche privilegi elevate) in modo sicuro, tracciato e modulare.

## Pillars
- Voce nativa: modalità comando vocale (push‑to‑talk o hotword) per eseguire azioni CLI, macro e flussi.
- Super‑plugin system: API unificata per provider (AI, shell actions, integrazioni VCS, pacchetti, theming DE, ecc.).
- Elevazione controllata: capacità di operare come amministratore solo dopo consenso esplicito (password / PAM) con sessione temporizzata e audit.
- Generazione contenuti desktop: generazione e applicazione temi per GNOME, KDE e altri Desktop Environment (roadmap) + in futuro Windows (schemi colore, terminal, accent colors, cursori, icone).
- UX centrata sulla chiarezza: comandi prevedibili, help auto‑generato, suggerimenti contestuali.
- Osservabilità: tracing strutturato, estendibile a OpenTelemetry.

## Visione a Breve
- Riconoscimento vocale offline/ibrido (es. Vosk / Whisper tiny) + fallback cloud opzionale.
- Interprete intenzioni (intent router) pluggable (pattern + LLM).
- Gestione sicura credenziali (keyring / secret store) + policy granulari per plugin privilegiati.
- Motore generazione temi: estrazione palette da immagine / prompt → generazione asset (GTK CSS, KDE color schemes, terminal profiles, wallpaper pipeline).

## Architettura (High Level)
```
+-------------------+          +-------------------+
|       CLI         | <------> |   Voice Gateway   |
+---------+---------+          +---------+---------+
          |                               |
          v                               v
    +-----------+                 +--------------+
    |  Core     |  <------------> | Plugins API  |
    +-----+-----+                 +------+-------+
          |                              |
   +------+------+          +------------+-------------+
   |  AI Providers |        |  System / Theming / VCS  |
   +---------------+        +--------------------------+
```
- crates/core: domain, orchestrazione, permission elevation, audit.
- crates/plugins-api: interfacce e tipi condivisi (traits, events, capability descriptors).
- crates/plugins/git_helper: esempio integrazione VCS.
- crates/ai-providers: adapter verso differenti motori (LLM / STT / TTS).
- crates/cli: parsing comandi (clap), modalità interattive e voice.

## Modalità Comando Vocale
- Hotword opzionale (es. "zen") o tasto (space / F9) configurabile.
- Streaming parziale → early intent detection.
- Cancella / annulla comando vocale ("annulla" / "cancel").
- Output parlato opzionale (TTS) o solo text.

## Sicurezza & Permessi
- Nessuna operazione privilegiata di default.
- Flow di elevazione: richiesta → prompt password (non memorizzata) → sessione privilegiata time-boxed (es. 5 min inattività) → revoca automatica.
- Audit log firmato (hash chain) per operazioni privilegiate.
- Policy YAML/TOML definisce: plugin, azione, arg pattern, richiede elevazione? tempo max? conferma vocale doppia?

## Theming Engine (Roadmap)
1. Palette Extraction: immagine / prompt → palette (algoritmo k‑means + refine contrasto WCAG).
2. Mapping: palette → semantic slots (background, accent, warning, highlight, selection).
3. Renderers:
   - GNOME: gtk.css, gsettings schema values.
   - KDE: .colors, kvantum, plasma look-and-feel (parziale).
   - Terminal: Alacritty / GNOME Terminal profiles.
   - (Futuro) Windows: accent color, terminal schemes, PowerShell profile.
4. Preview: TUI anteprima + export directory.

## Roadmap (Sintesi)
- [ ] Voice input MVP (Whisper tiny + push-to-talk)
- [ ] Intent router baseline (regex + fallback LLM small)
- [ ] Elevation manager + audit log
- [ ] Plugin capability registry & discovery
- [ ] Theming palette extraction + GNOME renderer
- [ ] KDE renderer
- [ ] Terminal schemes generator
- [ ] Policy engine (YAML) + tests
- [ ] Config layering (default + user + workspace)
- [ ] OpenTelemetry exporter (feature flag)
- [ ] Windows theming POC

Dettaglio esteso: vedi CHANGELOG e sezione Contributing.

## Installazione (Early Dev)
Prerequisiti: Rust stable (>=1.78), cargo.
```
git clone https://github.com/linuxiano85/ZenTerm-v2
cd ZenTerm-v2
cargo build --workspace
```
Esecuzione CLI (se crate cli presente):
```
cargo run -p cli -- --help
```

## Configurazione Iniziale
Vedi examples/config.example.toml. Copia in ~/.config/zentrm/config.toml e adatta.

## Contributi
Leggi CONTRIBUTING.md per linee guida (branching, commit, plugin design, sicurezza).

## Stato Licenza
File LICENSE attuale: BSD 2-Clause.
Campo manifest workspace: "MIT" (da riallineare). Proposta: dual-licensing "MIT OR BSD-2-Clause". Aprire issue se preferisci una sola licenza.

## Badge (verranno attivi dopo primo push CI)
![CI](https://github.com/linuxiano85/ZenTerm-v2/actions/workflows/ci.yml/badge.svg)

## Domande / Discussioni
Apri una issue (template in arrivo) o avvia una Discussion.

---
Semplifica. Estendi. Automatizza. Questo è ZenTerm v2.
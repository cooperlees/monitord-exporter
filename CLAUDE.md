# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**monitord-exporter** is a Prometheus exporter that exposes systemd health metrics collected by the `monitord` library. It listens for Prometheus scrape requests and collects stats on-demand via D-Bus, exporting metrics for networkd interfaces, PID 1 process stats, per-service stats, system state, and unit counts.

## Build & Development Commands

```bash
cargo build                          # Debug build
cargo build --release --all-features # Release build (optimized for size via LTO)
cargo test                           # Run tests
cargo test <test_name>               # Single test
cargo clippy                         # Lint
cargo fmt --check                    # Format check
cargo fmt                            # Auto-format
cargo run -- -p 9090 -s ssh.service -l debug  # Run locally on port 9090 monitoring ssh
```

## Architecture

Single-crate project (both library and binary). Four source files:

- `main.rs` — CLI parsing (clap derive), signal handling, Prometheus exporter startup, and the main request loop. Blocks on `exporter.wait_request()`, then calls `monitord::stat_collector()` via a tokio runtime to collect fresh stats per scrape.
- `lib.rs` — Library root, exports `metrics` and `logging` modules.
- `metrics.rs` — Core file (~670 lines). Defines `MonitordPromStats` which holds all Prometheus `GaugeVec` metrics organized into: `NetworkdStats` (per-interface), `Pid1Stats`, `ServiceStats` (per-service), `SystemStats`, and `UnitStats` (18 unit type counters). The `populate()` method maps `monitord::MonitordStats` into Prometheus gauges.
- `logging.rs` — Logging setup using tracing-glog, reuses `monitord::logging::LogLevels`.

### Request Flow

1. Prometheus scrapes the HTTP endpoint (binds to `[::]:port`)
2. `exporter.wait_request()` unblocks
3. `monitord::stat_collector()` collects systemd stats via D-Bus
4. `MonitordPromStats::populate()` maps stats into Prometheus gauges
5. Guard drops, metrics served to Prometheus

## Code Conventions

- Always run `cargo fmt` before committing
- Async collection via tokio, but main loop is synchronous (blocks on scrape requests)
- Error handling: `anyhow::Result` for top-level
- No rustfmt.toml or clippy.toml — default settings apply
- Release profile strips symbols and uses `opt-level = "z"` (size optimization)
- Prefer IPv6 where ever possible (exporter binds to `[::]`)
- Use as little deps as possible
- Keep the binary size down so we are an option for embedded systems

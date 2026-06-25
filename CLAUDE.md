# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

`tracing-subscriber-init` is a Rust library crate that provides a `TracingConfig` trait and convenience functions to reduce boilerplate when initializing `tracing-subscriber`. MSRV is 1.88.0, edition 2024.

## Commands

### Build
```sh
cargo build
cargo build --all-features
```

### Test
```sh
# Run all tests (preferred — uses nextest)
cargo nextest run

# Run all feature combinations
cargo matrix nextest run

# Run a single test by name
cargo nextest run <test_name>

# Run tests with a specific feature
cargo nextest run --features json
```

### Lint
```sh
cargo fmt
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -Dwarnings

# Via cargo-matrix (matches CI)
cargo matrix clippy --all-targets -- -Dwarnings
```

### Documentation
```sh
cargo +nightly doc --all-features
```

### Full pipeline (via cargo-rake)
```sh
cargo rake          # fmt → clippy → build → test → coverage
cargo rake most     # same as default
cargo rake all      # puc + most + clean
```

### Coverage
```sh
cargo matrix -F unstable llvm-cov nextest --no-report
cargo llvm-cov report --lcov --output-path lcov.info
cargo llvm-cov report --html
```

## Architecture

The crate has four concerns, one per module:

**`config.rs` — `TracingConfig` trait**
Users implement this trait on their own config structs. It exposes `quiet()` and `verbose()` (both `u8`) plus optional overrides for formatting options (`with_ansi`, `with_target`, `with_thread_ids`, etc.). The `quiet`/`verbose` counts map to `tracing::Level` via `get_effective_level` in `utils.rs`, with different behavior in debug vs. release builds.

**`src/format/` — layer constructors**
Four submodules (`compact`, `full`, `json` [feature-gated], `pretty`) each expose two public functions:
- `foo(config)` → `(fmt::Layer<S, ...>, LevelFilter)` — returns the layer and filter separately so callers can customize the layer (e.g., swap the writer) before attaching the filter.
- `filtered(config)` → `Filtered<fmt::Layer<S, ...>, LevelFilter, S>` — convenience wrapper that pre-attaches the filter.

**`initialize.rs` — subscriber registration**
Three functions (`init`, `try_init`, `set_default`) that accept `Vec<Box<dyn Layer<Registry> + Send + Sync>>`, build a `Registry`, attach the layers, and call the corresponding `tracing-subscriber` method.

**`utils.rs` — helpers**
- `get_effective_level(quiet, verbose)`: debug builds default to `INFO` at 0/0; release builds default to `ERROR` at 0/0.
- `TestAll`: a public `TracingConfig` impl with all options enabled, exported for use in downstream integration tests.

## Feature Flags

| Feature    | Enables |
|------------|---------|
| `json`     | JSON formatter (`json` / `json_filtered`) via `tracing-subscriber/json` |
| `tstime`   | Time formatters (`OffsetTime`, `UtcTime`, etc.) and re-exports of `time` well-known formats |
| `unstable` | Additional nightly-only lints (requires nightly toolchain) |

## Build Script & Lint Configuration

`build.rs` uses the `rustversion` crate to detect nightly and emit `cfg(nightly)`. When on nightly, `lib.rs` activates an exhaustive `deny()` list covering rustc, clippy, and rustdoc lints. The `unstable` feature adds a further set of nightly-unstable lints. All feature-matrix CI runs use `--all-features`; the `unstable` feature is only activated for coverage runs.

## CI

Tests run across MSRV (1.88.0), stable, beta, and nightly on Linux, macOS, and Windows (GNU + MSVC). Clippy runs only on nightly. Coverage uses `cargo-llvm-cov` with `cargo-nextest` on nightly.

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`releasor` is a Rust CLI tool that automates Rust release workflows: builds a release binary, creates a `.tar.gz` archive, generates a SHA-256 checksum, and copies it to the clipboard.

## Commands

This project uses [mise](https://mise.jdx.dev/) as a task runner. All tasks are defined in `mise.toml`.

```bash
mise build       # cargo build (debug)
mise release     # cargo build --release
mise test        # cargo test
mise lint        # cargo clippy --all-targets --all-features
mise fmt         # cargo fmt --all -- --check
mise check       # cargo check
mise cli         # cargo run -- -f releasor (test run)
```

Run a single test by name:
```bash
cargo test cli_tests::validate_project_name_empty -- --exact
```

Run all tests in a module:
```bash
cargo test cli_tests
```

## Architecture

The codebase is small (~320 lines) with clear single-responsibility modules:

- **`main.rs`** — Entry point; calls `Cli::start_release()`
- **`cli.rs`** — Core orchestration. `Cli::start_release()` parses CLI args (`-f <package_name>`), validates the name, then runs the 4-stage release workflow with animated progress
- **`output_command.rs`** — Executes external shell commands (`cargo build --release`, `tar`, `shasum`)
- **`status.rs`** — Wraps command results; prints success (✅) or exits with error (❌); has quiet variants for use during progress animation
- **`progress.rs`** — Animated terminal progress bar using atomic state and a background thread

### Release Workflow (4 stages)

Each stage animates the progress bar from the previous % to the next:

| Stage | Progress | Action |
|-------|----------|--------|
| 1 | 0→25% | `cargo build --release` |
| 2 | 25→50% | `tar -cvzf [name].tar.gz -C target/release [binary]` |
| 3 | 50→75% | `shasum -a 256 [name].tar.gz` |
| 4 | 75→100% | Copy checksum to clipboard via `arboard` |

### Tests

Integration and unit tests live in `tests/`:
- `cli_tests.rs` — Name validation, tar naming, shasum parsing
- `progress_tests.rs` — Progress bar rendering and animation
- `status_tests.rs` — Status check output and exit behavior
- `output_command_tests.rs` — Full workflow integration test

## Toolchain & Config

- Rust toolchain pinned in `rust-toolchain.toml` (currently 1.94.1)
- Formatter config in `.rustfmt.toml` (hard tabs, conventional import grouping)
- Changelog via `git cliff` with conventional commits (`mise changelog`)
- Releases are triggered by pushing a commit to `main` with message containing `"Prepare version to"` — CI extracts the version and creates a git tag

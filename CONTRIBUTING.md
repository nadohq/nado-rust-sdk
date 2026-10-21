# Contributing to the Nado Protocol Rust SDK

Thanks for your interest in contributing! Nado is building decentralized trading infrastructure on Ink, and community contributions — whether code, examples, or documentation — help make the SDK better for everyone.

This guide will help you get set up and submit your first contribution.

---

## Table of Contents

- [Ways to Contribute](#ways-to-contribute)
- [Getting Started](#getting-started)
- [Running Locally](#running-locally)
- [Submitting a Pull Request](#submitting-a-pull-request)
- [Reporting Bugs or Suggesting Features](#reporting-bugs-or-suggesting-features)
- [Code Style](#code-style)
- [Community](#community)

---

## Ways to Contribute

You don't have to write code to make a meaningful contribution. Here are some ways to help:

- **Bug reports** — Found something that doesn't work? Open an issue with steps to reproduce.
- **Documentation** — Spot something unclear in the README or code comments? Improvements are always welcome.
- **Examples** — New usage examples (e.g. querying positions, batch cancellations) help other developers onboard faster.
- **Feature requests** — Have an idea for a new SDK method or helper? Open an issue to discuss it first.
- **Code contributions** — Bug fixes, new features, or performance improvements via pull request.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- A funded wallet on Ink testnet (for execute sanity checks)
- An RPC endpoint for Ink (testnet or mainnet)

### Fork and Clone

```bash
# 1. Fork the repo on GitHub, then clone your fork
git clone https://github.com/<your-username>/nado-rust-sdk.git
cd nado-rust-sdk

# 2. Add the upstream remote to stay in sync
git remote add upstream https://github.com/nadohq/nado-rust-sdk.git
```

### Set Up Your Environment

Copy the example env file and fill in your private key:

```bash
cp .env.example .env
# Edit .env and add your RUST_SDK_PRIVATE_KEY
```

> ⚠️ **Never commit your `.env` file or expose your private key.** The `.gitignore` already excludes `.env` for you.

---

## Running Locally

### Build the project

```bash
cargo build
```

### Run sanity checks

These verify that the SDK behaves correctly against the Nado API:

```bash
# Query sanity (no signer required)
cargo run -- --query-sanity

# Execute sanity (requires signer in .env)
cargo run -- --execute-sanity

# Indexer sanity
cargo run -- --indexer-sanity
```

### Run the WebSocket example

```bash
cargo run --example place_order_websocket --features ws
```

---

## Submitting a Pull Request

1. **Create a branch** from `main` with a descriptive name:
   ```bash
   git checkout -b docs/improve-readme
   # or
   git checkout -b fix/cancel-order-error-handling
   ```

2. **Make your changes.** Keep commits focused — one logical change per commit.

3. **Run the sanity checks** to confirm nothing is broken.

4. **Push to your fork and open a PR** against the `main` branch of this repo.

5. **Fill out the PR description** with:
   - What changed and why
   - Any relevant issue numbers (e.g. `Closes #12`)
   - Steps to test your changes

The maintainers will review your PR and may request changes before merging.

---

## Reporting Bugs or Suggesting Features

Please [open a GitHub Issue](https://github.com/nadohq/nado-rust-sdk/issues) and include:

**For bugs:**
- SDK version (from `Cargo.toml`)
- Network (`test` or `prod`)
- Steps to reproduce
- Expected vs. actual behaviour
- Any relevant error messages or logs

**For feature requests:**
- What you're trying to do
- Why the current SDK doesn't support it
- Any ideas on how it could work

---

## Code Style

- Follow standard Rust idioms and formatting. Run `cargo fmt` before committing.
- Run `cargo clippy` and address any warnings where reasonable.
- Add doc comments (`///`) to any public functions or types you introduce.
- Keep public API changes backward-compatible where possible.

---

## Community

- **Docs:** [docs.nado.xyz](https://docs.nado.xyz)
- **API Reference:** [docs.nado.xyz/developer-resources/api](https://docs.nado.xyz/developer-resources/api/)
- **Nado on X:** [@nadoHQ](https://x.com/nadoHQ)
- **Ink Chain:** [inkonchain.com](https://inkonchain.com)

For general questions about the protocol, reach out via Nado's official channels above rather than opening a GitHub issue.

---

We appreciate every contribution, big or small. Welcome to the Nado community! 🌊

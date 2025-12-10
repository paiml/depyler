# Corpus Convergence Protocol

Run the convergence protocol from `docs/prompts/converge_reprorusted_100.md`.

## Quick Start

Use the built-in converge command with rich progress feedback:

```bash
# Build release binary first
cargo build --release --bin depyler

# Run convergence with TUI progress
./target/release/depyler converge --input-dir /home/noah/src/reprorusted-python-cli/examples --target-rate 80 --display rich --verbose
```

## Alternative: UTOL Automated Loop

```bash
./target/release/depyler utol --corpus /home/noah/src/reprorusted-python-cli/examples --target-rate 0.80 --display rich
```

## Quick Status Check

```bash
./target/release/depyler utol --corpus /home/noah/src/reprorusted-python-cli/examples --status
```

## Key Commands

| Task | Command |
|------|---------|
| Cache Warm | `depyler cache warm --input-dir $CORPUS` |
| Converge | `depyler converge --input-dir $CORPUS --target-rate 80` |
| UTOL | `depyler utol --corpus $CORPUS --target-rate 0.80` |
| Status | `depyler utol --corpus $CORPUS --status` |
| Explain | `depyler explain <file.rs> --trace <trace.json>` |

## Full Protocol

See: `docs/prompts/converge_reprorusted_100.md`

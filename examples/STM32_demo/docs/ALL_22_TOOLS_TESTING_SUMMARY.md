# MCP Tool Hardware Validation Notes

This file records a historical hardware validation pass for the STM32 RTT demo.
It is not a release guarantee and should not be treated as evidence for every
probe, target, operating system, or firmware image.

## Scope

- Target used: STM32G431CBTx
- Probe used: ST-Link V2
- Firmware: `examples/STM32_demo`
- Host interface: embedded-debugger-mcp MCP tools
- Main focus: probe discovery, target session lifecycle, memory operations,
  breakpoints, flash operations, RTT communication, and session cleanup

## Tool Areas Covered

| Area | Notes |
|------|-------|
| Probe management | Probe listing, session connection, and probe information were exercised. |
| Memory operations | Read/write flows were exercised against the validation target. |
| Debug control | Halt, run, reset, and step flows were exercised. |
| Breakpoints | Hardware breakpoint set and clear flows were exercised. |
| Flash operations | Program, erase, and verify flows were exercised on the validation target. |
| RTT communication | Up-channel reads, down-channel writes, channel listing, attach, and detach were exercised. |
| Session management | Disconnect and reconnect flows were exercised. |

## Important Limits

- Results are hardware-specific and should be re-run for each release candidate.
- Flash and memory operations are destructive on real targets; use an explicit
  config that enables the requested operation.
- Current release checks verify build, package, CLI, skill, and demo firmware
  compilation. They do not replay this full hardware validation automatically.
- Do not infer device success from this document alone. Record fresh command or
  MCP tool output when validating a board.

## Reproducible Baseline

Build the demo firmware:

```bash
cd examples/STM32_demo
CARGO_TARGET_DIR=/tmp/embedded-debugger-mcp-stm32-target cargo +nightly check --locked
```

Run host-side release checks from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --locked --all-features --no-deps
cargo package --locked
embedded-debugger-mcp doctor --json
embedded-debugger-mcp probes list --json
embedded-debugger-mcp skill print-prompt
```

## Follow-Up Validation

For a new target or release candidate, capture:

- exact binary or firmware path
- config file used for flash, memory, and file-path permissions
- probe identifier and target chip name
- command or MCP tool outputs for every destructive operation
- failure output when a forbidden operation is correctly rejected

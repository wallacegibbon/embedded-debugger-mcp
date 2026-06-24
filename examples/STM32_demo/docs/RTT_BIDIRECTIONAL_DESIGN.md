# RTT Bidirectional Demo Design

This document describes the RTT demo firmware shape used to exercise MCP RTT
read/write behavior. It is a design note for the example, not a release
certification record.

## Goals

- Expose multiple RTT up channels for terminal, data, and debug output.
- Expose RTT down channels for command and configuration input.
- Keep command parsing simple enough for manual and MCP-driven validation.
- Provide deterministic text output that can be captured in validation logs.

## Channel Layout

| Direction | Channel | Intended Use |
|-----------|---------|--------------|
| Up | 0 | Human-readable terminal output |
| Up | 1 | Periodic data stream |
| Up | 2 | Debug/status messages |
| Down | 0 | Short command input |
| Down | 1 | Configuration input |

## Command Model

The demo accepts small ASCII commands for validation:

| Command | Purpose |
|---------|---------|
| `L` | Toggle LED state |
| `R` | Reset demo counters |
| `F` | Report current Fibonacci value |
| `I` | Print system information |
| `SPEED:n` | Adjust demo speed multiplier |
| `LED:ON` / `LED:OFF` | Set LED state |
| `MODE:AUTO` / `MODE:MANUAL` | Change calculation mode |
| `RESET` | Reset demo state |

## Validation Expectations

A validation run should prove:

- RTT attach discovers the expected channels for the running firmware.
- Host reads receive text from up channels.
- Host writes reach down channels and trigger visible output.
- Invalid commands produce bounded error text rather than blocking the firmware.
- Detach and reconnect do not require restarting the host process.

## Limits

- Channel counts depend on the flashed firmware image.
- RTT control block discovery depends on target state and memory map.
- Hardware validation must record fresh command output for the specific board.
- The CI job checks that the demo firmware compiles; it does not require a
  connected target.

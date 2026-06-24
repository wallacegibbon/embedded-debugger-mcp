# RTT Hardware Validation Notes

This file summarizes a historical RTT validation pass for the STM32 demo. It
should be refreshed when validating a release candidate or a new board.

## Environment

- Target used: STM32G431CBTx
- Probe used: ST-Link V2
- Firmware: `examples/STM32_demo`
- Host: embedded-debugger-mcp MCP server

## Validation Areas

| Area | Expected Evidence |
|------|-------------------|
| Firmware deployment | Flash command output, reset output, and target state after reset |
| RTT attach | Channel counts and attach result |
| RTT read | Captured text from terminal, data, and debug channels |
| RTT write | Command output that proves down-channel input reached firmware |
| Configuration channel | Output showing configuration input was parsed |
| Detach/reconnect | Clean detach followed by successful attach |

## Commands Exercised

| Command | Purpose |
|---------|---------|
| `L` | Toggle LED state |
| `R` | Reset demo counters |
| `F` | Report current Fibonacci value |
| `I` | Print system information |
| `SPEED:n` | Set speed multiplier |
| `LED:ON` / `LED:OFF` | Control LED state |
| `MODE:AUTO` / `MODE:MANUAL` | Set calculation mode |
| `RESET` | Reset demo state |

## Release Interpretation

- Treat these notes as example validation history only.
- Do not claim hardware success for a new release without fresh output.
- Record rejected destructive operations as evidence when testing safety gates.
- CI checks compileability of the firmware example, not live target behavior.

## Suggested Fresh Evidence

For future validation, capture command output for:

```text
list_probes
connect
flash_program
rtt_attach
rtt_channels
rtt_read
rtt_write
rtt_detach
disconnect
```

Include the config file and target identifier with the run record.

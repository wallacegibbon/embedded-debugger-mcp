# STM32 RTT Bidirectional Communication Demo

![STM32G4 Development Board](img/stm32g4.jpg)

This example firmware demonstrates SEGGER RTT bidirectional communication with
an STM32 target and embedded-debugger-mcp.

## Hardware Requirements

- STM32G431CBTx or a similar STM32 board.
- Debug probe: ST-Link V2/V3, SEGGER J-Link, or CMSIS-DAPLink compatible probe.
- SWD wiring: SWDIO, SWCLK, GND, and target voltage reference.
- USB power or external target power.

## Build Check

This firmware uses `build-std`, so run it with nightly Rust and `rust-src`.

```bash
rustup component add rust-src --toolchain nightly
cd examples/STM32_demo
CARGO_TARGET_DIR=/tmp/embedded-debugger-mcp-stm32-target cargo +nightly check --locked
```

## RTT Channels

| Channel | Direction | Name | Purpose |
|---------|-----------|------|---------|
| Up 0 | Target to host | Terminal | System messages and command responses |
| Up 1 | Target to host | Data | Fibonacci calculation stream |
| Up 2 | Target to host | Debug | Status information |
| Down 0 | Host to target | Commands | Single-character commands |
| Down 1 | Host to target | Config | Multi-byte configuration commands |

## Interactive Commands

Channel 0 commands:

- `L`: toggle LED
- `R`: reset Fibonacci counter
- `F`: get current Fibonacci value
- `I`: print system information
- `0` to `9`: set calculation speed

Channel 1 examples:

- `SPEED:3`: set speed multiplier
- `LED:ON`: request LED on
- `MODE:AUTO`: request automatic mode

## Using With embedded-debugger-mcp

Typical MCP flow:

1. `list_probes`
2. `connect`
3. `flash_program` or `run_firmware`
4. `rtt_attach`
5. `rtt_channels`
6. `rtt_read` and `rtt_write`
7. `disconnect`

The demo is useful for hardware validation, RTT channel discovery, and command
round-trip checks. Results depend on the target board, probe, wiring, firmware
build, and local probe-rs support.

## Additional Notes

The `docs/` directory contains historical design and test notes from development.
Treat those files as development evidence, not as current release guarantees.

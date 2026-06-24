# Embedded Debugger Examples

This directory contains example firmware and notes for exercising
embedded-debugger-mcp with real hardware.

## STM32_demo

[STM32_demo](STM32_demo/) is an RTT bidirectional communication demo for an
STM32G431CBTx-class target.

It demonstrates:

- 5 RTT channels: 3 up channels and 2 down channels.
- Interactive command/response over RTT.
- Fibonacci data streaming with runtime control.
- A target firmware that can be used while testing MCP flash, debug, and RTT
  workflows.

Hardware used during development: STM32G431CBTx with ST-Link V2.

Build check:

```bash
cd STM32_demo
CARGO_TARGET_DIR=/tmp/embedded-debugger-mcp-stm32-target cargo +nightly check --locked
```

See [STM32_demo/README.md](STM32_demo/README.md).

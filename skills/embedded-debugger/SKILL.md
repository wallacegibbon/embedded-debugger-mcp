---
name: embedded-debugger
description: Embedded hardware debugging workflow for probe-rs targets using embedded-debugger-mcp. Use when Codex or Claude Code needs to inspect debug probes, validate embedded debugger setup, start the MCP server, guide a user through ARM Cortex-M/RISC-V flashing/debugging/RTT workflows, or operate without installing an MCP client by using the CLI plus prompts.
---

# Embedded Debugger

Use the local `embedded-debugger-mcp` binary as the source of truth. Prefer CLI
checks first, then MCP tools when an MCP client is available.

## Entry Decision

1. If the user has an MCP client configured, start or verify the server:
   `embedded-debugger-mcp serve`
2. If the user wants no MCP install, use CLI-first mode:
   `embedded-debugger-mcp doctor`, `embedded-debugger-mcp probes list`, and
   `embedded-debugger-mcp skill print-prompt`.
3. If hardware access is required, confirm the probe and target are connected
   before destructive actions such as flash erase or program.

## CLI Workflow

Run these in order and report the exact outcome:

```bash
embedded-debugger-mcp doctor
embedded-debugger-mcp probes list
embedded-debugger-mcp config show
```

Use JSON for automation:

```bash
embedded-debugger-mcp doctor --json
embedded-debugger-mcp probes list --json
```

## MCP Workflow

Use MCP tools for session-based operations:

1. `list_probes`
2. `connect`
3. Read-only checks such as `probe_info`, `get_status`, and `read_memory`
4. Mutating operations only after the user confirms target, file path, and risk:
   `write_memory`, `flash_erase`, `flash_program`, `run_firmware`
5. RTT operations after firmware is running: `rtt_attach`, `rtt_channels`,
   `rtt_read`, `rtt_write`, `rtt_detach`
6. `disconnect`

## Safety Rules

- Treat flash erase, flash program, memory write, reset, run, and RTT write as
  mutating hardware operations.
- Prefer read-only discovery before mutation.
- Respect project configuration limits for file paths, file sizes, memory
  ranges, and flash erase permissions.
- Do not claim hardware success from command text alone; cite the command or MCP
  tool result that produced the evidence.

## Prompt Reference

For a reusable CLI+Skill prompt, read
`references/default-prompt.md`.

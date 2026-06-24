use rmcp::{
    model::*, service::RequestContext, tool_handler, ErrorData as McpError, RoleServer,
    ServerHandler,
};
use tracing::info;

use super::session::EmbeddedDebuggerToolHandler;

#[tool_handler]
impl ServerHandler for EmbeddedDebuggerToolHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Embedded debugging and flash programming MCP server for ARM Cortex-M, RISC-V, and other probe-rs-supported targets. Exposes 22 tools for probe detection, target sessions, memory operations, breakpoints, RTT communication, and flash programming: list_probes, connect, disconnect, probe_info, halt, run, reset, step, get_status, read_memory, write_memory, set_breakpoint, clear_breakpoint, rtt_attach, rtt_detach, rtt_read, rtt_write, rtt_channels, flash_erase, flash_program, flash_verify, run_firmware.".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        info!("Embedded Debugger MCP server initialized with 22 tools (18 debug + 4 flash)");
        Ok(self.get_info())
    }
}

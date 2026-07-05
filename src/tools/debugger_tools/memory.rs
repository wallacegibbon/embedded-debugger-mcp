use rmcp::{handler::server::wrapper::Parameters, model::*, tool, tool_router, ErrorData as McpError};
use tracing::{debug, error, info};

use super::formatting::{format_memory_data, parse_address, parse_data};
use super::session::EmbeddedDebuggerToolHandler;
use crate::tools::types::*;
use probe_rs::MemoryInterface;

#[tool_router(router = memory_tool_router, vis = "pub")]
impl EmbeddedDebuggerToolHandler {
    // =============================================================================
    // Memory Operation Tools (2 tools)
    // =============================================================================

    #[tool(description = "Read memory from the target")]
    async fn read_memory(
        &self,
        Parameters(args): Parameters<ReadMemoryArgs>,
    ) -> Result<CallToolResult, McpError> {
        debug!(
            "Reading memory for session: {} at address {}",
            args.session_id, args.address
        );

        // Parse address
        let address = match parse_address(&args.address) {
            Ok(addr) => addr,
            Err(e) => {
                error!("Invalid address '{}': {}", args.address, e);
                return Err(McpError::internal_error(
                    format!("Invalid address '{}': {}", args.address, e),
                    None,
                ));
            }
        };

        let session_arc = self.get_session(&args.session_id).await?;
        self.ensure_memory_read_allowed(&session_arc, address, args.size)?;

        // Read memory
        {
            let mut session = session_arc.session.lock().await;
            let mut core = match session.core(0) {
                Ok(core) => core,
                Err(e) => {
                    error!("Failed to get core for session {}: {}", args.session_id, e);
                    return Err(McpError::internal_error(
                        format!("Failed to get core: {}", e),
                        None,
                    ));
                }
            };

            let mut data = vec![0u8; args.size];
            match core.read(address, &mut data) {
                Ok(_) => {
                    debug!("Read {} bytes from address 0x{:08X}", data.len(), address);

                    let formatted_data = format_memory_data(&data, &args.format, address);
                    let message = format!(
                        "Memory read completed successfully.\n\n\
                        Session ID: {}\n\
                        Address: 0x{:08X}\n\
                        Size: {} bytes\n\
                        Format: {}\n\n\
                        Data:\n{}",
                        args.session_id, address, args.size, args.format, formatted_data
                    );

                    info!("Memory read completed for session: {}", args.session_id);
                    Ok(CallToolResult::success(vec![ContentBlock::text(message)]))
                }
                Err(e) => {
                    error!(
                        "Failed to read memory for session {}: {}",
                        args.session_id, e
                    );
                    Err(McpError::internal_error(
                        format!("Failed to read memory: {}", e),
                        None,
                    ))
                }
            }
        }
    }

    #[tool(description = "Write memory to the target")]
    async fn write_memory(
        &self,
        Parameters(args): Parameters<WriteMemoryArgs>,
    ) -> Result<CallToolResult, McpError> {
        debug!(
            "Writing memory for session: {} at address {}",
            args.session_id, args.address
        );

        // Parse address
        let address = match parse_address(&args.address) {
            Ok(addr) => addr,
            Err(e) => {
                error!("Invalid address '{}': {}", args.address, e);
                return Err(McpError::internal_error(
                    format!("Invalid address '{}': {}", args.address, e),
                    None,
                ));
            }
        };

        // Parse data based on format
        let data = match parse_data(&args.data, &args.format) {
            Ok(data) => data,
            Err(e) => {
                error!("Invalid data '{}': {}", args.data, e);
                return Err(McpError::internal_error(
                    format!("Invalid data '{}': {}", args.data, e),
                    None,
                ));
            }
        };

        let session_arc = self.get_session(&args.session_id).await?;
        self.ensure_memory_write_allowed(&session_arc, address, data.len())?;

        // Write memory
        {
            let mut session = session_arc.session.lock().await;
            let mut core = match session.core(0) {
                Ok(core) => core,
                Err(e) => {
                    error!("Failed to get core for session {}: {}", args.session_id, e);
                    return Err(McpError::internal_error(
                        format!("Failed to get core: {}", e),
                        None,
                    ));
                }
            };

            match core.write(address, &data) {
                Ok(_) => {
                    let message = format!(
                        "Memory write completed successfully.\n\n\
                        Session ID: {}\n\
                        Address: 0x{:08X}\n\
                        Data: {}\n\
                        Format: {}\n\
                        Bytes written: {}",
                        args.session_id,
                        address,
                        args.data,
                        args.format,
                        data.len()
                    );

                    info!("Memory write completed for session: {}", args.session_id);
                    Ok(CallToolResult::success(vec![ContentBlock::text(message)]))
                }
                Err(e) => {
                    error!(
                        "Failed to write memory for session {}: {}",
                        args.session_id, e
                    );
                    Err(McpError::internal_error(
                        format!("Failed to write memory: {}", e),
                        None,
                    ))
                }
            }
        }
    }

    // =============================================================================
    // Breakpoint Tools (2 tools)
    // =============================================================================

    #[tool(description = "Set a breakpoint at the specified address")]
    async fn set_breakpoint(
        &self,
        Parameters(args): Parameters<SetBreakpointArgs>,
    ) -> Result<CallToolResult, McpError> {
        debug!(
            "Setting breakpoint for session: {} at address {}",
            args.session_id, args.address
        );

        if args.breakpoint_type != "hardware" {
            return Err(McpError::internal_error(
                format!(
                    "Unsupported breakpoint_type '{}'. Only hardware breakpoints are implemented.",
                    args.breakpoint_type
                ),
                None,
            ));
        }

        // Parse address
        let address = match parse_address(&args.address) {
            Ok(addr) => addr,
            Err(e) => {
                error!("Invalid address '{}': {}", args.address, e);
                return Err(McpError::internal_error(
                    format!("Invalid address '{}': {}", args.address, e),
                    None,
                ));
            }
        };

        let session_arc = self.get_session(&args.session_id).await?;

        // Set breakpoint
        {
            let mut session = session_arc.session.lock().await;
            let mut core = match session.core(0) {
                Ok(core) => core,
                Err(e) => {
                    error!("Failed to get core for session {}: {}", args.session_id, e);
                    return Err(McpError::internal_error(
                        format!("Failed to get core: {}", e),
                        None,
                    ));
                }
            };

            match core.set_hw_breakpoint(address) {
                Ok(_) => {
                    let message = format!(
                        "Breakpoint set successfully.\n\n\
                        Session ID: {}\n\
                        Address: 0x{:08X}\n\
                        Type: Hardware breakpoint\n\n\
                        The target will halt when execution reaches this address.",
                        args.session_id, address
                    );

                    info!(
                        "Breakpoint set for session: {} at 0x{:08X}",
                        args.session_id, address
                    );
                    Ok(CallToolResult::success(vec![ContentBlock::text(message)]))
                }
                Err(e) => {
                    error!(
                        "Failed to set breakpoint for session {}: {}",
                        args.session_id, e
                    );
                    Err(McpError::internal_error(
                        format!("Failed to set breakpoint: {}", e),
                        None,
                    ))
                }
            }
        }
    }

    #[tool(description = "Clear a breakpoint at the specified address")]
    async fn clear_breakpoint(
        &self,
        Parameters(args): Parameters<ClearBreakpointArgs>,
    ) -> Result<CallToolResult, McpError> {
        debug!(
            "Clearing breakpoint for session: {} at address {}",
            args.session_id, args.address
        );

        // Parse address
        let address = match parse_address(&args.address) {
            Ok(addr) => addr,
            Err(e) => {
                error!("Invalid address '{}': {}", args.address, e);
                return Err(McpError::internal_error(
                    format!("Invalid address '{}': {}", args.address, e),
                    None,
                ));
            }
        };

        let session_arc = {
            let sessions = self.sessions.read().await;
            match sessions.get(&args.session_id) {
                Some(session) => session.clone(),
                None => {
                    let error_msg = format!("Session '{}' not found\n\nUse 'connect' to establish a debug session first", args.session_id);
                    return Err(McpError::internal_error(error_msg, None));
                }
            }
        };

        // Clear breakpoint
        {
            let mut session = session_arc.session.lock().await;
            let mut core = match session.core(0) {
                Ok(core) => core,
                Err(e) => {
                    error!("Failed to get core for session {}: {}", args.session_id, e);
                    return Err(McpError::internal_error(
                        format!("Failed to get core: {}", e),
                        None,
                    ));
                }
            };

            match core.clear_hw_breakpoint(address) {
                Ok(_) => {
                    let message = format!(
                        "Breakpoint cleared successfully.\n\n\
                        Session ID: {}\n\
                        Address: 0x{:08X}\n\n\
                        The breakpoint has been removed.",
                        args.session_id, address
                    );

                    info!(
                        "Breakpoint cleared for session: {} at 0x{:08X}",
                        args.session_id, address
                    );
                    Ok(CallToolResult::success(vec![ContentBlock::text(message)]))
                }
                Err(e) => {
                    error!(
                        "Failed to clear breakpoint for session {}: {}",
                        args.session_id, e
                    );
                    Err(McpError::internal_error(
                        format!("Failed to clear breakpoint: {}", e),
                        None,
                    ))
                }
            }
        }
    }
}

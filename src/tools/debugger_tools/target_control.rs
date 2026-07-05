use rmcp::{handler::server::wrapper::Parameters, model::*, tool, tool_router, ErrorData as McpError};
use tracing::{debug, error, info, warn};

use super::session::EmbeddedDebuggerToolHandler;
use crate::tools::types::*;
use probe_rs::{CoreStatus, RegisterValue};

#[tool_router(router = target_control_tool_router, vis = "pub")]
impl EmbeddedDebuggerToolHandler {
    // =============================================================================
    // Target Control Tools (5 tools)
    // =============================================================================

    #[tool(description = "Halt the target CPU execution")]
    async fn halt(
        &self,
        Parameters(args): Parameters<HaltArgs>,
    ) -> Result<CallToolResult, McpError> {
        debug!("Halting target for session: {}", args.session_id);

        let session_arc = self.get_session(&args.session_id).await?;

        // Halt the target
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

            match core.halt(std::time::Duration::from_millis(1000)) {
                Ok(_) => {
                    // Get status after halt
                    match core.status() {
                        Ok(_status) => {
                            let pc = core
                                .read_core_reg(core.program_counter())
                                .map(|v: RegisterValue| v.try_into().unwrap_or(0u32))
                                .unwrap_or(0);
                            let sp = core
                                .read_core_reg(core.stack_pointer())
                                .map(|v: RegisterValue| v.try_into().unwrap_or(0u32))
                                .unwrap_or(0);

                            let message = format!(
                                "Target halted successfully!\n\n\
                                Session ID: {}\n\
                                PC: 0x{:08X}\n\
                                SP: 0x{:08X}\n\
                                State: Halted\n",
                                args.session_id, pc, sp
                            );

                            info!("Halt completed for session: {}", args.session_id);
                            Ok(CallToolResult::success(vec![ContentBlock::text(message)]))
                        }
                        Err(e) => {
                            warn!("Failed to get status after halt: {}", e);
                            let message = format!(
                                "Target halted successfully!\n\n\
                                Session ID: {}\n\
                                State: Halted\n",
                                args.session_id
                            );
                            Ok(CallToolResult::success(vec![ContentBlock::text(message)]))
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to halt target for session {}: {}",
                        args.session_id, e
                    );
                    Err(McpError::internal_error(
                        format!("Failed to halt target: {}", e),
                        None,
                    ))
                }
            }
        }
    }

    #[tool(description = "Resume target CPU execution")]
    async fn run(&self, Parameters(args): Parameters<RunArgs>) -> Result<CallToolResult, McpError> {
        debug!("Running target for session: {}", args.session_id);

        let session_arc = self.get_session(&args.session_id).await?;

        // Resume the target
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

            match core.run() {
                Ok(_) => {
                    let message = format!(
                        "Target resumed execution successfully!\n\n\
                        Session ID: {}\n\
                        Status: Running\n\n\
                        The target is now executing code. Use 'halt' to stop execution.",
                        args.session_id
                    );

                    info!("Run completed for session: {}", args.session_id);
                    Ok(CallToolResult::success(vec![ContentBlock::text(message)]))
                }
                Err(e) => {
                    error!(
                        "Failed to run target for session {}: {}",
                        args.session_id, e
                    );
                    Err(McpError::internal_error(
                        format!("Failed to run target: {}", e),
                        None,
                    ))
                }
            }
        }
    }

    #[tool(description = "Reset the target CPU")]
    async fn reset(
        &self,
        Parameters(args): Parameters<ResetArgs>,
    ) -> Result<CallToolResult, McpError> {
        debug!("Resetting target for session: {}", args.session_id);

        if args.reset_type != "hardware" {
            return Err(McpError::internal_error(
                format!(
                    "Unsupported reset_type '{}'. probe-rs core reset is exposed as 'hardware' by this server.",
                    args.reset_type
                ),
                None,
            ));
        }

        let session_arc = self.get_session(&args.session_id).await?;

        // Reset the target
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

            let reset_result = if args.halt_after_reset {
                core.reset_and_halt(std::time::Duration::from_millis(
                    self.config.debugger.connection_timeout_ms,
                ))
                .map(|_| ())
            } else {
                core.reset()
            };

            match reset_result {
                Ok(_) => {
                    let pc = core
                        .read_core_reg(core.program_counter())
                        .map(|v: RegisterValue| v.try_into().unwrap_or(0u32))
                        .unwrap_or(0);
                    let sp = core
                        .read_core_reg(core.stack_pointer())
                        .map(|v: RegisterValue| v.try_into().unwrap_or(0u32))
                        .unwrap_or(0);

                    let message = format!(
                        "Target reset completed successfully.\n\n\
                        Session ID: {}\n\
                        Reset type: {}\n\
                        Halted after reset: {}\n\
                        PC: 0x{:08X}\n\
                        SP: 0x{:08X}\n\
                        State: {}\n",
                        args.session_id,
                        args.reset_type,
                        args.halt_after_reset,
                        pc,
                        sp,
                        if args.halt_after_reset {
                            "Halted"
                        } else {
                            "Running"
                        }
                    );

                    info!("Reset completed for session: {}", args.session_id);
                    Ok(CallToolResult::success(vec![ContentBlock::text(message)]))
                }
                Err(e) => {
                    error!(
                        "Failed to reset target for session {}: {}",
                        args.session_id, e
                    );
                    Err(McpError::internal_error(
                        format!("Failed to reset target: {}", e),
                        None,
                    ))
                }
            }
        }
    }

    #[tool(description = "Execute a single instruction step")]
    async fn step(
        &self,
        Parameters(args): Parameters<StepArgs>,
    ) -> Result<CallToolResult, McpError> {
        debug!("Single stepping target for session: {}", args.session_id);

        let session_arc = self.get_session(&args.session_id).await?;

        // Single step the target
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

            match core.step() {
                Ok(_) => {
                    let pc = core
                        .read_core_reg(core.program_counter())
                        .map(|v: RegisterValue| v.try_into().unwrap_or(0u32))
                        .unwrap_or(0);
                    let sp = core
                        .read_core_reg(core.stack_pointer())
                        .map(|v: RegisterValue| v.try_into().unwrap_or(0u32))
                        .unwrap_or(0);

                    let message = format!(
                        "Single step completed successfully!\n\n\
                        Session ID: {}\n\
                        PC: 0x{:08X}\n\
                        SP: 0x{:08X}\n\
                        State: Halted\n",
                        args.session_id, pc, sp
                    );

                    info!("Step completed for session: {}", args.session_id);
                    Ok(CallToolResult::success(vec![ContentBlock::text(message)]))
                }
                Err(e) => {
                    error!(
                        "Failed to step target for session {}: {}",
                        args.session_id, e
                    );
                    Err(McpError::internal_error(
                        format!("Failed to step target: {}", e),
                        None,
                    ))
                }
            }
        }
    }

    #[tool(description = "Get current status of the target CPU and debug session")]
    async fn get_status(
        &self,
        Parameters(args): Parameters<GetStatusArgs>,
    ) -> Result<CallToolResult, McpError> {
        debug!("Getting status for session: {}", args.session_id);

        let session_arc = self.get_session(&args.session_id).await?;

        // Get target status
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

            match core.status() {
                Ok(status) => {
                    let pc = core
                        .read_core_reg(core.program_counter())
                        .map(|v: RegisterValue| v.try_into().unwrap_or(0u32))
                        .unwrap_or(0);
                    let sp = core
                        .read_core_reg(core.stack_pointer())
                        .map(|v: RegisterValue| v.try_into().unwrap_or(0u32))
                        .unwrap_or(0);

                    let is_halted = matches!(status, CoreStatus::Halted(_));
                    let halt_reason = match status {
                        CoreStatus::Halted(reason) => format!("{:?}", reason),
                        CoreStatus::Running => "N/A".to_string(),
                        _ => "Unknown".to_string(),
                    };

                    let message = format!(
                        "Debug Session Status\n\n\
                        Core Information:\n\
                        - PC: 0x{:08X}\n\
                        - SP: 0x{:08X}\n\
                        - State: {}\n\
                        - Halt reason: {}\n\n\
                        Session Information:\n\
                        - ID: {}\n\
                        - Connected: true\n\
                        - Target: {}\n\
                        - Probe: {}\n\
                        - Duration: {:.1} minutes\n",
                        pc,
                        sp,
                        if is_halted { "Halted" } else { "Running" },
                        halt_reason,
                        args.session_id,
                        session_arc.target_chip,
                        session_arc.probe_identifier,
                        (chrono::Utc::now() - session_arc.created_at).num_seconds() as f64 / 60.0
                    );

                    Ok(CallToolResult::success(vec![ContentBlock::text(message)]))
                }
                Err(e) => {
                    error!(
                        "Failed to get core status for session {}: {}",
                        args.session_id, e
                    );
                    Err(McpError::internal_error(
                        format!("Failed to get core status: {}", e),
                        None,
                    ))
                }
            }
        }
    }
}

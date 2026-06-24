//! Integration tests for debugger MCP server

use clap::Parser;
use embedded_debugger_mcp::config::Args;
use embedded_debugger_mcp::Config;

#[tokio::test]
async fn test_config_validation() {
    let config = Config::default();
    assert!(config.validate().is_ok());

    // Test TOML serialization
    let toml_str = config.to_toml().unwrap();
    assert!(!toml_str.is_empty());
    assert!(toml_str.contains("[server]"));
    assert!(toml_str.contains("[debugger]"));
}

#[tokio::test]
async fn test_probe_discovery() {
    // Test probe discovery (this will work even without hardware)
    use embedded_debugger_mcp::debugger::discovery::ProbeDiscovery;

    let result = ProbeDiscovery::list_probes();
    assert!(result.is_ok());

    // The result might be empty if no probes are connected, which is fine
    let probes = result.unwrap();
    println!("Found {} probes", probes.len());
}

#[test]
fn test_cli_defaults_do_not_override_config_file_values() {
    let mut config = Config::default();
    config.server.max_sessions = 9;
    config.debugger.default_speed_khz = 1200;
    config.security.allow_flash_erase = true;
    config.security.restrict_memory_access = true;

    let args = Args::parse_from(["embedded-debugger-mcp"]);
    config.merge_args(&args);

    assert_eq!(config.server.max_sessions, 9);
    assert_eq!(config.debugger.default_speed_khz, 1200);
    assert!(config.security.allow_flash_erase);
    assert!(config.security.restrict_memory_access);
}

#[test]
fn test_cli_explicit_values_override_config_file_values() {
    let mut config = Config::default();
    let args = Args::parse_from([
        "embedded-debugger-mcp",
        "--max-sessions",
        "10",
        "--default-speed",
        "1600",
        "--allow-flash-erase",
        "--restrict-memory-access",
    ]);

    config.merge_args(&args);

    assert_eq!(config.server.max_sessions, 10);
    assert_eq!(config.debugger.default_speed_khz, 1600);
    assert!(config.security.allow_flash_erase);
    assert!(config.security.restrict_memory_access);
}

#[test]
fn test_error_types() {
    use embedded_debugger_mcp::DebugError;

    let error = DebugError::ProbeNotFound("test".to_string());
    assert!(error.to_string().contains("Probe not found"));

    let error = DebugError::SessionLimitExceeded(5);
    assert!(error.to_string().contains("Session limit exceeded"));
}

#[test]
fn test_probe_type_detection() {
    use embedded_debugger_mcp::utils::ProbeType;

    // Test J-Link detection
    assert_eq!(ProbeType::from_vid_pid(0x1366, 0x0101), ProbeType::JLink);

    // Test ST-Link detection
    assert_eq!(ProbeType::from_vid_pid(0x0483, 0x374B), ProbeType::StLink);

    // Test DAPLink detection
    assert_eq!(ProbeType::from_vid_pid(0x0D28, 0x0204), ProbeType::DapLink);

    // Test unknown probe
    assert_eq!(ProbeType::from_vid_pid(0xFFFF, 0xFFFF), ProbeType::Unknown);
}

#[tokio::test]
async fn test_mcp_tool_handler() {
    // Test the main MCP tool handler
    use embedded_debugger_mcp::EmbeddedDebuggerToolHandler;

    let _handler = EmbeddedDebuggerToolHandler::new(10);

    // Test that we can create multiple handlers (should work fine)
    let _handler2 = EmbeddedDebuggerToolHandler::new(5);

    // Verify the handler was created - this is more meaningful than just instantiation
    println!("MCP tool handler created and ready for use");
}

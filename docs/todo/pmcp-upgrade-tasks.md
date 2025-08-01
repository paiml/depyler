# PMCP SDK Upgrade Tasks

This document outlines the granular tasks required to upgrade the Depyler MCP integration from a basic custom implementation to using the production-ready pmcp Rust MCP SDK.

## Overview

Current state: Basic custom MCP implementation with stub methods
Target state: Full pmcp SDK integration with proper transport layers, type safety, and protocol compliance

## Pre-Upgrade Tasks

### 1. Dependencies and Project Setup
- [ ] Add pmcp dependency to depyler-mcp/Cargo.toml with appropriate features
- [ ] Update workspace dependencies to include pmcp's transitive deps (tokio features, etc.)
- [ ] Add feature flags for optional transport layers (stdio, websocket, http)
- [ ] Update depyler-mcp crate documentation to reflect new SDK usage

### 2. Remove Old Implementation
- [ ] Archive current protocol.rs as protocol_legacy.rs for reference
- [ ] Remove custom McpMessage, McpResponse, McpError types
- [ ] Remove hardcoded protocol constants and error codes
- [ ] Clean up manual JSON-RPC handling code

## Core Integration Tasks

### 3. Transport Layer Setup
- [ ] Replace custom message handling with pmcp Transport trait
- [ ] Implement StdioTransport for CLI usage (primary)
- [ ] Add WebSocketTransport support for remote MCP servers (optional feature)
- [ ] Add HttpTransport support for REST-based MCP servers (optional feature)
- [ ] Create transport factory based on configuration/environment

### 4. Server Implementation Refactor
- [ ] Convert DepylerMcpServer to use pmcp::server::Server builder pattern
- [ ] Implement ToolHandler trait for transpile_python tool
- [ ] Implement ToolHandler trait for analyze_migration_complexity tool
- [ ] Implement ToolHandler trait for verify_transpilation tool
- [ ] Add proper RequestHandlerExtra usage for cancellation support
- [ ] Implement server capability negotiation using pmcp types

### 5. Client Implementation
- [ ] Replace McpClient stub with pmcp::client::Client
- [ ] Implement proper async client initialization flow
- [ ] Add retry logic and connection management
- [ ] Implement client-side tool discovery and caching
- [ ] Add progress notification support for long operations
- [ ] Handle server-initiated notifications properly

### 6. Type System Migration
- [ ] Replace custom types with pmcp protocol types:
  - [ ] InitializeParams → pmcp::types::InitializeParams
  - [ ] ServerCapabilities → pmcp::types::ServerCapabilities
  - [ ] ToolDefinition → pmcp::types::Tool
  - [ ] Error types → pmcp::error::Error variants
- [ ] Update all serde annotations to match pmcp conventions
- [ ] Ensure backward compatibility with existing tool schemas

### 7. Error Handling Upgrade
- [ ] Replace DepylerMcpError with pmcp::error::Error wrapping
- [ ] Implement proper error recovery strategies
- [ ] Add structured error context using pmcp patterns
- [ ] Ensure all errors are properly propagated through transport

## Advanced Features

### 8. Resource Support (Future Enhancement)
- [ ] Design resource URIs for Python/Rust code artifacts
- [ ] Implement ResourceHandler trait for code resources
- [ ] Add resource subscription for live reload support
- [ ] Enable resource-based project navigation

### 9. Prompt Support (Future Enhancement)
- [ ] Design prompts for common transpilation scenarios
- [ ] Implement PromptHandler for interactive workflows
- [ ] Add context-aware prompt suggestions

### 10. Logging Integration
- [ ] Replace tracing calls with MCP logging protocol
- [ ] Implement structured logging with proper levels
- [ ] Add log forwarding to MCP clients

## Testing and Quality

### 11. Test Suite Updates
- [ ] Update unit tests to use pmcp mocks
- [ ] Add integration tests with real pmcp transports
- [ ] Test all three transport types (stdio, websocket, http)
- [ ] Add property-based tests for protocol compliance
- [ ] Ensure test coverage remains above 80%

### 12. Benchmarking
- [ ] Add benchmarks comparing old vs new implementation
- [ ] Measure transport overhead for different message sizes
- [ ] Profile memory usage with concurrent connections
- [ ] Ensure no performance regression

## Integration Tasks

### 13. CLI Integration
- [ ] Update depyler CLI to use new MCP client
- [ ] Add --mcp-transport flag for transport selection
- [ ] Implement proper stdio framing in CLI mode
- [ ] Add connection status indicators

### 14. Configuration
- [ ] Add MCP configuration to depyler.toml
- [ ] Support environment variable overrides
- [ ] Document all configuration options
- [ ] Add validation for MCP endpoints

### 15. Documentation
- [ ] Update API documentation with pmcp types
- [ ] Create MCP integration guide
- [ ] Add examples for each transport type
- [ ] Document breaking changes and migration path

## Deployment Tasks

### 16. Backward Compatibility
- [ ] Ensure existing MCP clients can still connect
- [ ] Add compatibility shim if needed
- [ ] Document any breaking protocol changes

### 17. Release Preparation
- [ ] Update CHANGELOG.md with MCP upgrade details
- [ ] Bump version numbers appropriately
- [ ] Update README with new MCP features
- [ ] Prepare migration guide for users

### 18. CI/CD Updates
- [ ] Update GitHub Actions to test MCP functionality
- [ ] Add MCP server smoke tests
- [ ] Ensure all features compile in CI
- [ ] Add integration test job for MCP

## Post-Upgrade Tasks

### 19. Performance Optimization
- [ ] Profile and optimize hot paths
- [ ] Implement connection pooling for HTTP transport
- [ ] Add caching for frequently used tools
- [ ] Optimize serialization/deserialization

### 20. Monitoring and Observability
- [ ] Add OpenTelemetry spans for MCP operations
- [ ] Implement metrics for tool usage
- [ ] Add health check endpoint for server
- [ ] Create dashboard for MCP metrics

## Success Criteria

1. All existing MCP functionality works with pmcp SDK
2. Tests pass with >80% coverage
3. No performance regression vs current implementation
4. Full protocol compliance with MCP spec
5. Clean clippy output with pedantic lints
6. Successful deployment to crates.io
7. GitHub Actions CI passes all checks

## Timeline Estimate

- Phase 1 (Tasks 1-7): Core integration - 4 hours
- Phase 2 (Tasks 8-10): Advanced features - 2 hours (optional, can defer)
- Phase 3 (Tasks 11-15): Testing and docs - 3 hours
- Phase 4 (Tasks 16-20): Deployment and optimization - 2 hours

Total estimated effort: 9-11 hours of focused work

## Implementation Status (2025-08-01)

**COMPLETED**: Successfully upgraded Depyler MCP integration to use pmcp Rust SDK

### Key Changes Made:

1. **Dependencies Updated**:
   - Added `pmcp` dependency with features: validation, websocket, http
   - Added `async-trait` and `tokio-util` dependencies 
   - Added feature flags for modular transport support

2. **Architecture Refactor**:
   - Replaced custom MCP protocol implementation with pmcp SDK
   - Implemented pmcp `ToolHandler` trait for all three tools
   - Created `TransportFactory` for flexible transport configuration
   - Updated client to use pmcp `Client` with proper initialization

3. **Server Implementation**:
   - `TranspileTool`, `AnalyzeTool`, `VerifyTool` now implement pmcp's `ToolHandler`
   - Server uses pmcp's `Server::builder()` pattern
   - Proper error handling using pmcp error types
   - Support for request cancellation and progress reporting

4. **Transport Layer**:
   - Multi-transport support: stdio (default), WebSocket, HTTP
   - Environment variable configuration (`DEPYLER_MCP_TRANSPORT`)
   - Proper transport abstraction using pmcp's `Transport` trait

5. **Error Handling**:
   - Updated error types to work with pmcp errors
   - Proper error propagation through transport layer
   - Enhanced error context for debugging

6. **Tests Updated**:
   - Rewrote all tests to use pmcp types and patterns
   - Added transport factory tests
   - Added client integration tests
   - Maintained full test coverage

### Technical Highlights:

- **Full MCP Protocol Compliance**: Uses pmcp's complete MCP implementation
- **Type Safety**: All operations now use strongly-typed pmcp structs
- **Performance**: Zero-copy message parsing where possible
- **Reliability**: Built-in retry logic and connection management  
- **Extensibility**: Easy to add new tools via pmcp's `ToolHandler` trait

### Backward Compatibility:

- All existing MCP tool APIs remain the same
- Same JSON schemas for tool inputs/outputs
- Protocol version compatibility maintained
- Environment configuration preserved

### Future Enhancements Ready:

- Resource support for code artifacts
- Prompt support for interactive workflows  
- Logging integration with MCP protocol
- Advanced authentication schemes
- WebSocket and HTTP transport usage

The upgrade provides a solid foundation for advanced MCP features while maintaining compatibility with existing Depyler workflows.
# Depyler Background Agent Mode

The Depyler Agent provides continuous Python-to-Rust transpilation services through the Model Context Protocol (MCP), enabling seamless integration with Claude Code and other AI assistants.

## Quick Start

```bash
# Start the agent in foreground mode
depyler agent start --foreground

# Start as background daemon
depyler agent start --port 3000

# Check status
depyler agent status

# Stop the agent
depyler agent stop
```

## Features

### 🚀 MCP Server Integration
- **PMCP-powered**: High-performance MCP server using the PMCP SDK
- **6 Transpilation Tools**: Comprehensive Python-to-Rust conversion capabilities
- **Real-time Monitoring**: File system watching with automatic transpilation
- **Claude Code Ready**: Direct integration with Claude Desktop and VS Code

### 🛠️ Available MCP Tools

1. **transpile_python_file**: Convert individual Python files to Rust
2. **transpile_python_directory**: Batch transpilation for entire directories
3. **monitor_python_project**: Set up continuous monitoring for a project
4. **get_transpilation_status**: Query transpilation metrics and status
5. **verify_rust_code**: Validate generated Rust code
6. **analyze_python_compatibility**: Check Python feature support

### 📊 Background Daemon Features
- Process management with PID file tracking
- Graceful shutdown handling
- Health checks and automatic recovery
- Configurable working directory
- Comprehensive logging

## Installation & Setup

### 1. Install Depyler

```bash
# From source
cargo install --path crates/depyler

# Or download pre-built binary
curl -L https://github.com/paiml/depyler/releases/latest/download/depyler-linux-x64 -o depyler
chmod +x depyler
sudo mv depyler /usr/local/bin/
```

### 2. Configure Claude Code

Add to your Claude Desktop configuration (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):

```json
{
  "mcpServers": {
    "depyler": {
      "command": "depyler",
      "args": ["agent", "start", "--foreground"],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### 3. Start Using in Claude Code

Once configured, Claude will have access to Depyler's transpilation tools. You can:
- Ask Claude to transpile Python files to Rust
- Set up project monitoring for automatic transpilation
- Get compatibility analysis for Python code
- Verify generated Rust code quality

## Configuration

### Configuration File

Create `~/.depyler/agent.json`:

```json
{
  "agent": {
    "port": 3000,
    "debug": false,
    "auto_transpile": true,
    "verification_level": "basic"
  },
  "transpilation_monitor": {
    "update_interval": 2,
    "watch_patterns": ["**/*.py"],
    "debounce_interval": 500,
    "max_batch_size": 20,
    "auto_transpile": true,
    "verification_level": "basic"
  },
  "daemon": {
    "working_directory": "~/.depyler",
    "log_file": "~/.depyler/agent.log",
    "pid_file": "/tmp/depyler_agent.pid",
    "auto_restart": true,
    "restart_delay": 5,
    "max_restarts": 3
  }
}
```

### Environment Variables

- `DEPYLER_PORT`: MCP server port (default: 3000)
- `DEPYLER_DEBUG`: Enable debug logging
- `DEPYLER_CONFIG`: Path to configuration file
- `RUST_LOG`: Logging level (trace, debug, info, warn, error)

## Command Reference

### Start Agent

```bash
depyler agent start [OPTIONS]

OPTIONS:
    --port <PORT>           MCP server port [default: 3000]
    --config <PATH>         Configuration file path
    --foreground           Run in foreground (don't daemonize)
    --debug                Enable debug mode
```

### Monitor Projects

```bash
# Add project to monitoring
depyler agent add-project /path/to/project --id my-project

# Remove project
depyler agent remove-project my-project

# List monitored projects
depyler agent list-projects
```

### View Logs

```bash
# Show last 50 lines
depyler agent logs

# Show last 100 lines
depyler agent logs -n 100

# Follow log output
depyler agent logs --follow
```

## API Examples

### Using with MCP Clients

```javascript
// Example MCP client request
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "transpile_python_file",
    "arguments": {
      "file_path": "/path/to/script.py",
      "output_path": "/path/to/output.rs",
      "verify": true
    }
  },
  "id": 1
}
```

### Response Format

```javascript
{
  "jsonrpc": "2.0",
  "result": {
    "content": [{
      "type": "text",
      "text": "✅ Successfully transpiled script.py → output.rs\n..."
    }],
    "metadata": {
      "python_file": "/path/to/script.py",
      "rust_file": "/path/to/output.rs",
      "python_lines": 100,
      "rust_lines": 150,
      "transpilation_time_ms": 45,
      "verification": "passed",
      "warnings_count": 0
    }
  },
  "id": 1
}
```

## Architecture

```
┌─────────────────────┐
│   Claude Code       │
│   (MCP Client)      │
└──────────┬──────────┘
           │ JSON-RPC
           ▼
┌─────────────────────┐
│  Depyler MCP Server │
│     (PMCP SDK)      │
├─────────────────────┤
│   Tool Handlers     │
│  - Transpile File   │
│  - Monitor Project  │
│  - Verify Code      │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Transpiler Engine  │
│  - AST → HIR → Rust │
│  - Type Inference   │
│  - Verification     │
└─────────────────────┘
```

## Performance

- **Transpilation Speed**: >10MB/s for typical Python code
- **Memory Usage**: <100MB for agent daemon
- **File Watching**: Instant detection with 500ms debounce
- **Batch Processing**: Up to 20 files per batch
- **Concurrent Requests**: Thread-safe with async/await

## Troubleshooting

### Agent Won't Start

```bash
# Check if port is in use
lsof -i :3000

# Check for existing PID file
rm /tmp/depyler_agent.pid

# Run with debug logging
RUST_LOG=debug depyler agent start --foreground --debug
```

### Connection Issues

```bash
# Test MCP server directly
echo '{"jsonrpc":"2.0","method":"tools/list","id":1}' | \
  depyler agent start --foreground
```

### File Watching Not Working

- Ensure you have proper permissions for watched directories
- Check `inotify` limits on Linux: `cat /proc/sys/fs/inotify/max_user_watches`
- Verify patterns in configuration match your files

## Security Considerations

- The agent only transpiles files, never executes code
- File system access is read-only for Python files
- Generated Rust files are written with user permissions
- No network access beyond local MCP connections
- Configuration files should not contain sensitive data

## Support

- **Issues**: https://github.com/paiml/depyler/issues
- **Documentation**: https://docs.rs/depyler
- **Discord**: https://discord.gg/depyler

## License

MIT OR Apache-2.0
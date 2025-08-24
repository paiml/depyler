# Depyler Troubleshooting Guide

## Common Issues and Solutions

### Agent Won't Start

#### Problem: Port already in use
```
Error: Address already in use (os error 48)
```

**Solution:**
```bash
# Check what's using port 3000
lsof -i :3000  # macOS/Linux
netstat -ano | findstr :3000  # Windows

# Use a different port
depyler agent start --port 3001
```

#### Problem: Permission denied
```
Error: Permission denied (os error 13)
```

**Solution:**
```bash
# Check file permissions
ls -la ~/.depyler/

# Fix permissions
chmod 755 ~/.depyler
chmod 644 ~/.depyler/agent.json

# Run without systemd integration
depyler agent start --foreground
```

#### Problem: PID file exists
```
Error: Daemon already running or stale PID file
```

**Solution:**
```bash
# Check if actually running
depyler agent status

# If not running, remove stale PID
rm /tmp/depyler_agent.pid

# Restart
depyler agent start
```

### Claude Code Integration Issues

#### Problem: Claude can't find Depyler
```
MCP server 'depyler' not found
```

**Solution:**
1. Ensure Depyler is in PATH:
```bash
which depyler
# If not found:
export PATH="$HOME/.local/bin:$PATH"
```

2. Verify configuration path:
- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Windows: `%APPDATA%\Claude\claude_desktop_config.json`
- Linux: `~/.config/Claude/claude_desktop_config.json`

3. Restart Claude Desktop after configuration changes

#### Problem: MCP connection timeout
```
Failed to connect to MCP server
```

**Solution:**
```bash
# Test MCP server directly
echo '{"jsonrpc":"2.0","method":"tools/list","id":1}' | depyler agent start --foreground

# Check firewall settings
sudo ufw status  # Linux
sudo pfctl -s rules  # macOS

# Try with debug logging
RUST_LOG=debug depyler agent start --foreground --debug
```

### Transpilation Errors

#### Problem: Unsupported Python feature
```
Error: Unsupported Python construct: <feature>
```

**Solution:**
- Check [supported features](README.md#supported-python-features)
- Simplify code to use basic Python constructs
- Use type annotations for better inference:
```python
def add(a: int, b: int) -> int:
    return a + b
```

#### Problem: Type inference failure
```
Error: Cannot infer type for variable
```

**Solution:**
```python
# Add type hints
x: int = 42
names: List[str] = ["Alice", "Bob"]

# Use explicit casts
result = int(calculate_value())
```

#### Problem: Ownership inference error
```
Error: Cannot determine ownership for value
```

**Solution:**
- Avoid complex aliasing patterns
- Use immutable patterns where possible
- Clone values explicitly when needed:
```python
original = [1, 2, 3]
copy = original.copy()  # Explicit copy
```

### File Monitoring Issues

#### Problem: File changes not detected
```
File monitoring not triggering transpilation
```

**Solution:**
```bash
# Check inotify limits (Linux)
cat /proc/sys/fs/inotify/max_user_watches
# Increase if needed:
echo fs.inotify.max_user_watches=524288 | sudo tee -a /etc/sysctl.conf
sudo sysctl -p

# Verify watch patterns
depyler agent add-project /path/to/project --patterns "**/*.py"

# Check debounce settings (may be too long)
# Edit ~/.depyler/agent.json
{
  "transpilation_monitor": {
    "debounce_interval": 500  // Reduce from default
  }
}
```

#### Problem: Too many files being watched
```
Warning: File watch limit exceeded
```

**Solution:**
- Use more specific patterns:
```bash
depyler agent add-project ./src --patterns "src/**/*.py"
# Instead of:
depyler agent add-project . --patterns "**/*.py"
```

- Exclude virtual environments:
```json
{
  "transpilation_monitor": {
    "exclude_patterns": ["**/venv/**", "**/.git/**", "**/node_modules/**"]
  }
}
```

### Performance Issues

#### Problem: High CPU usage
```
Agent consuming excessive CPU
```

**Solution:**
```bash
# Increase debounce interval
# Edit ~/.depyler/agent.json
{
  "transpilation_monitor": {
    "debounce_interval": 2000,  // 2 seconds
    "max_batch_size": 10  // Reduce batch size
  }
}

# Disable auto-transpilation
{
  "agent": {
    "auto_transpile": false
  }
}
```

#### Problem: Memory usage growing
```
Agent memory usage increasing over time
```

**Solution:**
```bash
# Restart periodically
depyler agent restart

# Enable memory limits (if using systemd)
# Edit /etc/systemd/system/depyler-agent.service
[Service]
MemoryLimit=500M
MemoryMax=500M

# Monitor with:
systemctl status depyler-agent
```

### Verification Failures

#### Problem: Generated Rust won't compile
```
Error: Generated Rust code fails compilation
```

**Solution:**
```bash
# Check Rust installation
rustc --version
cargo --version

# Try with basic verification only
depyler transpile input.py --verify=basic

# Check for missing dependencies
cargo check

# Manually fix common issues:
# - Add missing imports
# - Fix lifetime annotations
# - Add type annotations
```

#### Problem: Clippy warnings
```
Warning: Clippy found issues in generated code
```

**Solution:**
```bash
# Run clippy manually to see issues
cargo clippy -- -D warnings

# Common fixes:
# - Use .clone() explicitly
# - Add #[allow(dead_code)] for unused functions
# - Use &str instead of &String
```

## Diagnostic Commands

### Check System Status
```bash
# Agent status
depyler agent status

# View recent logs
depyler agent logs -n 100

# Follow logs in real-time
depyler agent logs --follow

# Check version
depyler --version

# Test transpilation
echo 'print("test")' | depyler transpile /dev/stdin
```

### Debug Mode
```bash
# Run with full debug output
RUST_LOG=trace depyler agent start --foreground --debug

# Save debug output
RUST_LOG=debug depyler agent start --foreground 2>&1 | tee debug.log

# Check configuration
cat ~/.depyler/agent.json | jq .
```

### Reset Everything
```bash
# Stop agent
depyler agent stop

# Remove configuration
rm -rf ~/.depyler

# Remove PID file
rm -f /tmp/depyler_agent.pid

# Reinstall
curl -sSL https://github.com/paiml/depyler/raw/main/install.sh | bash

# Start fresh
depyler agent start --foreground
```

## Getting Help

### Resources
- **GitHub Issues**: https://github.com/paiml/depyler/issues
- **Documentation**: https://docs.rs/depyler
- **Discord**: https://discord.gg/depyler
- **Examples**: `/examples` directory in repository

### Reporting Issues

When reporting issues, please include:

1. **System Information**:
```bash
depyler --version
rustc --version
uname -a  # OS info
```

2. **Error Messages**:
```bash
# Full error output
RUST_LOG=debug depyler agent start --foreground 2>&1 | head -100
```

3. **Configuration**:
```bash
cat ~/.depyler/agent.json
```

4. **Reproduction Steps**:
- Exact commands run
- Sample Python file (if applicable)
- Expected vs actual behavior

### Debug Checklist

- [ ] Depyler installed correctly: `which depyler`
- [ ] Rust toolchain installed: `rustc --version`
- [ ] Port available: `lsof -i :3000`
- [ ] Permissions correct: `ls -la ~/.depyler`
- [ ] No stale PID: `ls /tmp/depyler_agent.pid`
- [ ] PATH configured: `echo $PATH`
- [ ] Claude config valid: JSON syntax check
- [ ] Firewall allows localhost connections
- [ ] Sufficient disk space: `df -h`
- [ ] File watch limits adequate: `cat /proc/sys/fs/inotify/max_user_watches`
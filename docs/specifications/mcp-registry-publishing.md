# MCP Registry Publishing Specification

## Overview

Depyler is automatically published to the [Model Context Protocol (MCP) Registry](https://registry.modelcontextprotocol.io) to enable AI assistants like Claude to discover and use it as a transpilation server.

## Automated Publishing

### Trigger Mechanism

Publishing is triggered automatically on version tags:

```bash
git tag v3.4.0
git push origin v3.4.0
```

### Workflow Pipeline

The publishing workflow (`.github/workflows/publish-mcp-registry.yml`) executes:

1. **Run Tests** - Full test suite validation
2. **Publish to crates.io** - All workspace crates
3. **Wait for Availability** - 60s for crates.io indexing
4. **Publish to MCP Registry** - Using GitHub OIDC authentication
5. **Verify Publication** - Confirm server is live

### Authentication

- **Method**: GitHub OIDC (automatic in CI)
- **Permissions**: `id-token: write`, `contents: read`
- **Namespace**: `io.github.noahgift` (personal account)
- **No Secrets Required**: OIDC provides automatic organization access

## Server Configuration

### Registry Metadata

**Location**: `server.json`

```json
{
  "$schema": "https://static.modelcontextprotocol.io/schemas/2025-09-29/server.schema.json",
  "name": "io.github.noahgift/depyler-mcp",
  "displayName": "Depyler MCP Server",
  "description": "MCP server for Depyler: Python-to-Rust transpiler with analysis and verification tools",
  "version": "3.4.0",
  "homepage": "https://github.com/paiml/depyler",
  "sourceUrl": "https://github.com/paiml/depyler/tree/main",
  "author": {
    "name": "PAIML Team",
    "email": "team@paiml.com",
    "url": "https://github.com/paiml"
  },
  "license": "MIT",
  "readme": "https://raw.githubusercontent.com/paiml/depyler/main/README.md",
  "categories": ["development-tools", "compilers"],
  "tags": ["python", "rust", "transpiler", "compiler", "verification", "energy-efficient"],
  "deployment": {
    "type": "package",
    "package": {
      "type": "cargo",
      "name": "depyler",
      "binaryName": "depyler",
      "features": []
    }
  }
}
```

### Key Fields

- **name**: Unique identifier (`io.github.noahgift/depyler-mcp`)
- **displayName**: Human-readable name shown in registry
- **version**: Must match `Cargo.toml` workspace version
- **deployment.package**: Cargo installation configuration
- **categories/tags**: Discovery and search optimization

## Version Management

### Version Synchronization

Version must be consistent across:

1. `Cargo.toml` - `workspace.package.version = "3.4.0"`
2. `server.json` - `version: "3.4.0"`
3. Git tag - `v3.4.0`

### Update Process

```bash
# 1. Update Cargo.toml
vim Cargo.toml  # Set version = "3.4.0"

# 2. Update server.json
vim server.json  # Set version: "3.4.0"

# 3. Commit
git add Cargo.toml server.json
git commit -m "Bump version to 3.4.0"

# 4. Tag and push
git tag v3.4.0
git push origin main v3.4.0
```

## Registry URLs

- **Search**: https://registry.modelcontextprotocol.io/?search=depyler
- **API**: https://registry.modelcontextprotocol.io/v0/servers?search=depyler
- **Direct**: `io.github.noahgift/depyler-mcp`

## Installation for Users

```bash
# Install from crates.io
cargo install depyler

# Verify installation
depyler --version
```

## Claude Desktop Integration

Add to `~/.config/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "depyler": {
      "command": "depyler",
      "args": ["agent", "start", "--foreground"]
    }
  }
}
```

## Troubleshooting

### Common Issues

1. **Version Mismatch**
   - Ensure `Cargo.toml`, `server.json`, and git tag versions match
   - Re-tag if needed: `git tag -d v3.4.0 && git tag v3.4.0 && git push -f origin v3.4.0`

2. **Crates.io Publishing Fails**
   - Version already exists: Cannot republish same version
   - Solution: Increment version number

3. **MCP Registry Authentication**
   - GitHub OIDC requires `id-token: write` permission
   - Verify in `.github/workflows/publish-mcp-registry.yml`

4. **Server Not Found in Registry**
   - Wait 1-2 minutes for indexing
   - Check workflow logs: `gh run list --workflow="Publish to MCP Registry"`

## Schema Validation

Validate `server.json` locally:

```bash
curl -s https://static.modelcontextprotocol.io/schemas/2025-09-29/server.schema.json -o /tmp/schema.json
jsonschema -i server.json /tmp/schema.json
```

## CI/CD Workflow Details

**File**: `.github/workflows/publish-mcp-registry.yml`

**Jobs**:
1. `publish-to-crates` - Publishes all workspace crates sequentially
2. `publish-to-mcp-registry` - Publishes to MCP registry using OIDC

**Dependencies**:
- crates.io: Requires `CARGO_TOKEN` secret
- MCP Registry: Uses GitHub OIDC (no secrets needed)

## References

- **MCP Registry**: https://github.com/modelcontextprotocol/registry
- **Publishing Guide**: [docs/mcp-registry-publish.md](../mcp-registry-publish.md)
- **Schema Docs**: https://github.com/modelcontextprotocol/registry/blob/main/docs/reference/server-json/
- **Workflow Examples**: Based on ruchy, pforge, and pmat-agent patterns

## Success Criteria

✅ Server appears in registry search
✅ Status shows "active"
✅ Version matches release
✅ Installation via `cargo install` works
✅ Claude Desktop integration functional

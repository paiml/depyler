# MCP Registry Publishing Guide

## Overview

Depyler is published to the [Model Context Protocol (MCP) Registry](https://registry.modelcontextprotocol.io) to enable AI assistants like Claude to use it as a transpilation and analysis server.

## Automated Publishing Process

Publishing to the MCP registry is fully automated via GitHub Actions when you push a version tag.

### Prerequisites

1. **CARGO_TOKEN Secret**: Ensure the `CARGO_TOKEN` secret is configured in the GitHub repository
   - Go to: Settings → Secrets and variables → Actions → Repository secrets
   - The token should have publish permissions for crates.io

2. **GitHub OIDC**: No additional setup needed - authentication to MCP registry uses GitHub's built-in OIDC

### Publishing Steps

1. **Update Version** (if needed):
   ```bash
   # Version is managed in Cargo.toml workspace.package
   # Update version field, e.g., version = "3.4.0"
   ```

2. **Update server.json**:
   ```bash
   # Ensure version in server.json matches Cargo.toml
   # Located at: ./server.json
   ```

3. **Create and Push Tag**:
   ```bash
   git tag v3.4.0
   git push origin v3.4.0
   ```

4. **Automated Workflow**:
   - Workflow: `.github/workflows/publish-mcp-registry.yml`
   - Triggered on: `v*` tags
   - Steps:
     1. Runs all tests
     2. Publishes to crates.io (all workspace crates)
     3. Publishes to MCP registry
     4. Verifies publication

### Manual Trigger

You can also manually trigger the workflow:
1. Go to: Actions → "Publish to MCP Registry"
2. Click "Run workflow"
3. Select the tag/branch to run from

## Server Configuration

### server.json Metadata

The MCP registry configuration is in `server.json`:

```json
{
  "$schema": "https://static.modelcontextprotocol.io/schemas/2025-09-29/server.schema.json",
  "name": "io.github.paiml/depyler",
  "description": "Energy-efficient Python-to-Rust transpiler with MCP support",
  "repository": {
    "url": "https://github.com/paiml/depyler",
    "source": "github"
  },
  "version": "3.4.0",
  "websiteUrl": "https://github.com/paiml/depyler",
  "packages": [
    {
      "registryType": "mcpb",
      "identifier": "depyler",
      "version": "3.4.0",
      "transport": {
        "type": "stdio",
        "command": "depyler",
        "args": ["agent", "start", "--foreground"]
      }
    }
  ]
}
```

### Configuration Fields

- **name**: Reverse-DNS format namespace (`io.github.paiml/depyler`)
- **description**: Max 100 characters
- **version**: Must match Cargo.toml version
- **registryType**: `mcpb` (MCP Bundle) for binary installations
- **transport**: Communication protocol (stdio for CLI tools)

## Troubleshooting

### Common Issues

#### 1. Schema Validation Errors

**Error**: `deprecated schema detected`

**Solution**: Update `$schema` URL in server.json to latest:
```json
{
  "$schema": "https://static.modelcontextprotocol.io/schemas/2025-09-29/server.schema.json"
}
```

#### 2. Description Too Long

**Error**: `description is too long`

**Solution**: Keep description ≤100 characters

#### 3. Dependency Conflicts

**Error**: Build failures due to dependency incompatibilities

**Solution**:
```bash
# Update dependencies
cargo update

# Test build locally
cargo build --workspace --all-features

# If specific dependency issues, update in Cargo.toml
```

#### 4. Tests Failing in CI

**Error**: Tests fail during workflow

**Solution**:
```bash
# Run tests locally first
cargo test --workspace --all-features

# Fix any failing tests before pushing tag
```

#### 5. Publishing to crates.io Fails

**Error**: `error: failed to publish`

**Solution**:
- Verify CARGO_TOKEN secret is valid
- Ensure version doesn't already exist on crates.io
- Check all dependencies are published

#### 6. Authentication Failures

**Error**: MCP registry authentication fails

**Solution**:
- Workflow uses GitHub OIDC (automatic)
- Verify workflow permissions include `id-token: write`
- Check `.github/workflows/publish-mcp-registry.yml`:
  ```yaml
  permissions:
    id-token: write
    contents: read
  ```

### Workflow Debugging

View workflow logs:
```bash
# List recent runs
gh run list --workflow="Publish to MCP Registry"

# View specific run
gh run view <run-id> --log

# View failed steps only
gh run view <run-id> --log-failed
```

## Version Management

### Semantic Versioning

Follow [semver](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backwards compatible)
- **PATCH**: Bug fixes

### Version Update Checklist

- [ ] Update version in `Cargo.toml` (workspace.package.version)
- [ ] Update version in `server.json` (2 places: root and packages[0])
- [ ] Update `CHANGELOG.md`
- [ ] Commit changes
- [ ] Create and push tag matching version

### Re-publishing a Version

If you need to re-publish:

```bash
# Delete tag locally and remotely
git tag -d v3.4.0
git push origin :refs/tags/v3.4.0

# Fix issues, commit

# Recreate tag
git tag v3.4.0
git push origin v3.4.0
```

## Dependencies

### Required Tools in CI

The workflow automatically installs:
- Rust (stable)
- mcp-publisher CLI (latest)
- jq (JSON processing)

### Local Testing Tools

For local validation:
```bash
# Install JSON schema validator
pip install jsonschema

# Install mcp-publisher
curl -sL https://github.com/modelcontextprotocol/registry/releases/latest/download/mcp-publisher_*_linux_amd64.tar.gz | tar xz

# Validate server.json
jsonschema -i server.json <(curl -s https://static.modelcontextprotocol.io/schemas/2025-09-29/server.schema.json)
```

## Registry Resources

- **MCP Registry**: https://registry.modelcontextprotocol.io
- **Documentation**: https://github.com/modelcontextprotocol/registry
- **Schema**: https://static.modelcontextprotocol.io/schemas/2025-09-29/server.schema.json
- **Publishing Guide**: https://github.com/modelcontextprotocol/registry/blob/main/docs/guides/publishing/

## Verification

After publishing, verify the server is live:

```bash
# Search registry
curl "https://registry.modelcontextprotocol.io/v0/servers?search=depyler" | jq .

# Check specific version
curl "https://registry.modelcontextprotocol.io/v0/servers?name=io.github.paiml/depyler" | jq .
```

## Support

For issues with:
- **Depyler**: Open an issue at https://github.com/paiml/depyler/issues
- **MCP Registry**: Open an issue at https://github.com/modelcontextprotocol/registry/issues
- **Publishing Workflow**: Check workflow logs and this guide first

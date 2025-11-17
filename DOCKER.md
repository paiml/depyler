# Docker Support for Depyler Agent

Run Depyler Agent in Docker for isolated, reproducible deployments.

## Quick Start

### Using Docker Compose (Recommended)

```bash
# Start the agent
docker-compose up -d

# View logs
docker-compose logs -f depyler-agent

# Stop the agent
docker-compose down
```

### Using Docker CLI

```bash
# Build the image
docker build -f Dockerfile.agent -t depyler/agent:3.1.0 .

# Run the agent
docker run -d \
  --name depyler-agent \
  -p 3000:3000 \
  -v $(pwd)/examples:/home/depyler/projects/examples:ro \
  depyler/agent:3.1.0

# Check status
docker exec depyler-agent depyler agent status

# View logs
docker logs -f depyler-agent

# Stop and remove
docker stop depyler-agent
docker rm depyler-agent
```

## Configuration

### Environment Variables

Configure the agent via environment variables:

```yaml
environment:
  - RUST_LOG=info                    # Logging level: trace, debug, info, warn, error
  - DEPYLER_AUTO_TRANSPILE=true      # Enable automatic transpilation
  - DEPYLER_VERIFICATION_LEVEL=basic # Verification level: none, basic, strict
  - DEPYLER_PORT=3000                # MCP server port
  - DEPYLER_HOST=0.0.0.0            # Bind address
```

### Volume Mounts

Mount your Python projects for transpilation:

```yaml
volumes:
  # Mount read-only for safety
  - /path/to/python/project:/home/depyler/projects/myproject:ro
  
  # Mount configuration directory
  - ./config:/home/depyler/.depyler
  
  # Named volume for persistence
  - depyler-data:/home/depyler/.depyler
```

### Custom Configuration

Create a custom `agent.json`:

```json
{
  "agent": {
    "port": 3000,
    "host": "0.0.0.0",
    "auto_transpile": true
  },
  "transpilation_monitor": {
    "debounce_interval": 1000,
    "patterns": ["**/*.py"],
    "exclude_patterns": ["**/test_*.py"]
  },
  "mcp": {
    "enabled": true,
    "max_message_size": 10485760
  }
}
```

Mount it in the container:

```bash
docker run -d \
  -v $(pwd)/agent.json:/home/depyler/.depyler/agent.json:ro \
  depyler/agent:3.1.0
```

## Claude Code Integration

### Local Development

For Claude Code to connect to the Dockerized agent:

1. Ensure port 3000 is exposed:
```yaml
ports:
  - "3000:3000"  # Or "127.0.0.1:3000:3000" for localhost only
```

2. Update Claude Desktop config:
```json
{
  "mcpServers": {
    "depyler": {
      "command": "docker",
      "args": ["exec", "-i", "depyler-agent", "depyler", "agent", "start", "--foreground"],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### Network Bridge Mode

For better isolation, use the Claude bridge service:

```bash
# Start both services
docker-compose up -d

# Claude connects to port 3001 (MCP-only mode)
# Agent runs on port 3000 (full mode)
```

## Monitoring

### Health Checks

The container includes health checks:

```bash
# Check health status
docker inspect depyler-agent --format='{{.State.Health.Status}}'

# View health check logs
docker inspect depyler-agent --format='{{range .State.Health.Log}}{{.Output}}{{end}}'
```

### Logs

Access different log levels:

```bash
# Info level (default)
docker logs depyler-agent

# Debug level
docker run -e RUST_LOG=debug depyler/agent:3.1.0

# Trace level (verbose)
docker run -e RUST_LOG=trace depyler/agent:3.1.0

# Follow logs
docker logs -f depyler-agent

# Last 100 lines
docker logs --tail 100 depyler-agent
```

### Metrics

Monitor resource usage:

```bash
# Real-time stats
docker stats depyler-agent

# Detailed inspection
docker inspect depyler-agent

# Process list inside container
docker exec depyler-agent ps aux
```

## Production Deployment

### Resource Limits

Set resource constraints for production:

```yaml
services:
  depyler-agent:
    deploy:
      resources:
        limits:
          cpus: '2.0'
          memory: 1G
        reservations:
          cpus: '0.5'
          memory: 256M
```

### Restart Policy

Configure automatic restart:

```yaml
services:
  depyler-agent:
    restart: unless-stopped  # or 'always' for production
```

### Logging

Configure log rotation:

```yaml
services:
  depyler-agent:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

### Security

Run with security options:

```yaml
services:
  depyler-agent:
    security_opt:
      - no-new-privileges:true
    cap_drop:
      - ALL
    cap_add:
      - NET_BIND_SERVICE
    read_only: true
    tmpfs:
      - /tmp
```

## Multi-Platform Build

Build for multiple architectures:

```bash
# Setup buildx
docker buildx create --use

# Build for multiple platforms
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -f Dockerfile.agent \
  -t depyler/agent:3.1.0 \
  --push .
```

## Troubleshooting

### Container Won't Start

```bash
# Check logs
docker logs depyler-agent

# Debug mode
docker run -it --rm \
  -e RUST_LOG=debug \
  depyler/agent:3.1.0 \
  agent start --foreground --debug
```

### Permission Issues

```bash
# Check file permissions
docker exec depyler-agent ls -la /home/depyler/.depyler

# Run as root for debugging
docker run -it --rm --user root depyler/agent:3.1.0 bash
```

### Network Connectivity

```bash
# Test from inside container
docker exec depyler-agent curl http://localhost:3000/health

# Check port binding
docker port depyler-agent

# Inspect network
docker network inspect depyler-network
```

### Memory Issues

```bash
# Check memory usage
docker stats depyler-agent --no-stream

# Increase memory limit
docker update --memory 2g depyler-agent
```

## Docker Hub Deployment

### Publishing

```bash
# Tag for Docker Hub
docker tag depyler/agent:3.1.0 yourusername/depyler-agent:3.1.0

# Login to Docker Hub
docker login

# Push image
docker push yourusername/depyler-agent:3.1.0
```

### Pulling

```bash
# Pull from Docker Hub
docker pull depyler/agent:latest

# Run pulled image
docker run -d -p 3000:3000 depyler/agent:latest
```

## Kubernetes Deployment

### Basic Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: depyler-agent
spec:
  replicas: 1
  selector:
    matchLabels:
      app: depyler-agent
  template:
    metadata:
      labels:
        app: depyler-agent
    spec:
      containers:
      - name: depyler-agent
        image: depyler/agent:3.1.0
        ports:
        - containerPort: 3000
        env:
        - name: RUST_LOG
          value: "info"
        resources:
          limits:
            memory: "1Gi"
            cpu: "1000m"
          requests:
            memory: "256Mi"
            cpu: "250m"
---
apiVersion: v1
kind: Service
metadata:
  name: depyler-agent
spec:
  selector:
    app: depyler-agent
  ports:
  - port: 3000
    targetPort: 3000
  type: ClusterIP
```

### Apply to Kubernetes

```bash
kubectl apply -f k8s-deployment.yaml
kubectl get pods
kubectl logs -f deployment/depyler-agent
```

---

For more Docker examples and configurations, check the repository for additional resources.
# Multi-stage Dockerfile for Depyler Playground
FROM rust:1.75 as wasm-builder

# Install wasm-pack and tools
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN wget https://github.com/WebAssembly/binaryen/releases/download/version_116/binaryen-version_116-x86_64-linux.tar.gz && \
    tar xf binaryen-version_116-x86_64-linux.tar.gz && \
    cp binaryen-version_116/bin/wasm-opt /usr/local/bin/

WORKDIR /app

# Copy Rust workspace
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
COPY playground/depyler-playground/ ./playground/depyler-playground/

# Build WASM module
RUN cd crates/depyler-wasm && \
    wasm-pack build --target web --out-dir ../../playground/public/wasm --release

# Optimize WASM
RUN cd playground/public/wasm && \
    wasm-opt -Oz -o optimized.wasm depyler_wasm_bg.wasm && \
    mv optimized.wasm depyler_wasm_bg.wasm

# Frontend build stage
FROM denoland/deno:1.40.0 as frontend-builder

WORKDIR /app

# Copy frontend source
COPY playground/deno.json playground/deno.lock* ./
COPY playground/vite.config.ts playground/index.html ./
COPY playground/src/ ./src/
COPY playground/public/ ./public/

# Copy WASM artifacts from previous stage
COPY --from=wasm-builder /app/playground/public/wasm/ ./public/wasm/

# Install dependencies and build
RUN deno cache --lock=deno.lock src/main.tsx
RUN deno task build

# Production stage
FROM nginx:alpine

# Install curl for healthchecks
RUN apk add --no-cache curl

# Copy built assets
COPY --from=frontend-builder /app/dist/ /var/www/depyler-playground/

# Copy nginx configuration
COPY playground/nginx.conf /etc/nginx/conf.d/default.conf

# Remove default nginx config
RUN rm /etc/nginx/conf.d/default.conf

# Create nginx configuration
RUN cat > /etc/nginx/nginx.conf << 'EOF'
user nginx;
worker_processes auto;
error_log /var/log/nginx/error.log notice;
pid /var/run/nginx.pid;

events {
    worker_connections 1024;
    use epoll;
    multi_accept on;
}

http {
    include /etc/nginx/mime.types;
    default_type application/octet-stream;

    # Logging
    log_format main '$remote_addr - $remote_user [$time_local] "$request" '
                    '$status $body_bytes_sent "$http_referer" '
                    '"$http_user_agent" "$http_x_forwarded_for"';

    access_log /var/log/nginx/access.log main;

    # Performance optimizations
    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    keepalive_timeout 65;
    types_hash_max_size 2048;
    client_max_body_size 16M;

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_proxied any;
    gzip_comp_level 6;
    gzip_types
        text/plain
        text/css
        text/xml
        text/javascript
        application/json
        application/javascript
        application/xml+rss
        application/atom+xml
        image/svg+xml;

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
    limit_conn_zone $binary_remote_addr zone=conn_limit_per_ip:10m;

    # Include server configs
    include /etc/nginx/conf.d/*.conf;
}
EOF

# Create simple server config for container
RUN cat > /etc/nginx/conf.d/playground.conf << 'EOF'
server {
    listen 80;
    server_name _;
    root /var/www/depyler-playground;
    index index.html;

    # Security headers
    add_header X-Content-Type-Options nosniff always;
    add_header X-Frame-Options DENY always;
    add_header X-XSS-Protection "1; mode=block" always;

    # WASM configuration
    location ~ \.wasm$ {
        add_header Content-Type "application/wasm" always;
        add_header Cache-Control "public, max-age=31536000, immutable" always;
        gzip_static on;
    }

    # Static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2)$ {
        add_header Cache-Control "public, max-age=31536000, immutable" always;
        expires 1y;
        gzip_static on;
    }

    # API proxy (if needed)
    location /api/ {
        limit_req zone=api burst=20 nodelay;
        proxy_pass http://host.docker.internal:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Health check
    location /health {
        access_log off;
        add_header Content-Type text/plain;
        return 200 "OK";
    }

    # SPA fallback
    location / {
        try_files $uri $uri/ /index.html;
        
        location = /index.html {
            add_header Cache-Control "no-cache, no-store, must-revalidate" always;
            add_header Pragma "no-cache" always;
            add_header Expires "0" always;
        }
    }
}
EOF

EXPOSE 80

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost/health || exit 1

CMD ["nginx", "-g", "daemon off;"]
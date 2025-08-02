use anyhow::Result;
use pmcp::{StdioTransport, Transport};
use std::fmt::Debug;

#[cfg(feature = "websocket")]
use pmcp::WebSocketTransport;

#[cfg(feature = "http")]
use pmcp::HttpTransport;

#[derive(Debug, Clone, Default)]
pub enum TransportType {
    #[default]
    Stdio,
    #[cfg(feature = "websocket")]
    WebSocket(String), // URL
    #[cfg(feature = "http")]
    Http(String), // URL
}

pub struct TransportFactory;

impl TransportFactory {
    pub fn create_transport(transport_type: TransportType) -> Result<Box<dyn Transport>> {
        match transport_type {
            TransportType::Stdio => {
                let transport = StdioTransport::new();
                Ok(Box::new(transport))
            }

            #[cfg(feature = "websocket")]
            TransportType::WebSocket(url) => {
                use pmcp::WebSocketConfig;
                let config = WebSocketConfig {
                    url: url.parse().map_err(|e| anyhow::anyhow!("Invalid URL: {}", e))?,
                    auto_reconnect: true,
                    max_reconnect_attempts: Some(5),
                    max_reconnect_delay: std::time::Duration::from_secs(30),
                    ping_interval: Some(std::time::Duration::from_secs(30)),
                    request_timeout: std::time::Duration::from_secs(10),
                    reconnect_delay: std::time::Duration::from_secs(1),
                };
                let transport = WebSocketTransport::new(config);
                Ok(Box::new(transport))
            }

            #[cfg(feature = "http")]
            TransportType::Http(url) => {
                use pmcp::HttpConfig;
                let config = HttpConfig {
                    base_url: url.parse().map_err(|e| anyhow::anyhow!("Invalid URL: {}", e))?,
                    headers: vec![],
                    timeout: std::time::Duration::from_secs(30),
                    max_idle_per_host: 8,
                    enable_pooling: true,
                    sse_endpoint: None,
                };
                let transport = HttpTransport::new(config);
                Ok(Box::new(transport))
            }
        }
    }

    pub fn create_stdio() -> Result<StdioTransport> {
        Ok(StdioTransport::new())
    }

    #[cfg(feature = "websocket")]
    pub fn create_websocket(url: &str) -> Result<WebSocketTransport> {
        use pmcp::WebSocketConfig;
        let config = WebSocketConfig {
            url: url.parse().map_err(|e| anyhow::anyhow!("Invalid URL: {}", e))?,
            auto_reconnect: true,
            max_reconnect_attempts: Some(5),
            max_reconnect_delay: std::time::Duration::from_secs(30),
            ping_interval: Some(std::time::Duration::from_secs(30)),
            request_timeout: std::time::Duration::from_secs(10),
            reconnect_delay: std::time::Duration::from_secs(1),
        };
        Ok(WebSocketTransport::new(config))
    }

    #[cfg(feature = "http")]
    pub fn create_http(url: &str) -> Result<HttpTransport> {
        use pmcp::HttpConfig;
        let config = HttpConfig {
            base_url: url.parse().map_err(|e| anyhow::anyhow!("Invalid URL: {}", e))?,
            headers: vec![],
            timeout: std::time::Duration::from_secs(30),
            max_idle_per_host: 8,
            enable_pooling: true,
            sse_endpoint: None,
        };
        Ok(HttpTransport::new(config))
    }

    pub fn from_env() -> Result<TransportType> {
        if let Ok(mcp_transport) = std::env::var("DEPYLER_MCP_TRANSPORT") {
            match mcp_transport.as_str() {
                "stdio" => Ok(TransportType::Stdio),

                #[cfg(feature = "websocket")]
                url if url.starts_with("ws://") || url.starts_with("wss://") => {
                    Ok(TransportType::WebSocket(url.to_string()))
                }

                #[cfg(feature = "http")]
                url if url.starts_with("http://") || url.starts_with("https://") => {
                    Ok(TransportType::Http(url.to_string()))
                }

                _ => {
                    tracing::warn!("Unknown transport type in DEPYLER_MCP_TRANSPORT: {}, falling back to stdio", mcp_transport);
                    Ok(TransportType::Stdio)
                }
            }
        } else {
            Ok(TransportType::Stdio)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_transport_type() {
        assert!(matches!(TransportType::default(), TransportType::Stdio));
    }

    #[test]
    fn test_from_env_stdio() {
        std::env::set_var("DEPYLER_MCP_TRANSPORT", "stdio");
        let transport_type = TransportFactory::from_env().unwrap();
        assert!(matches!(transport_type, TransportType::Stdio));
        std::env::remove_var("DEPYLER_MCP_TRANSPORT");
    }

    #[test]
    fn test_from_env_default_when_not_set() {
        std::env::remove_var("DEPYLER_MCP_TRANSPORT");
        let transport_type = TransportFactory::from_env().unwrap();
        assert!(matches!(transport_type, TransportType::Stdio));
    }

    #[test]
    fn test_create_stdio_transport() {
        let transport = TransportFactory::create_stdio();
        assert!(transport.is_ok());
    }

    #[cfg(feature = "websocket")]
    #[test]
    fn test_from_env_websocket() {
        std::env::set_var("DEPYLER_MCP_TRANSPORT", "ws://localhost:8080");
        let transport_type = TransportFactory::from_env().unwrap();
        assert!(matches!(transport_type, TransportType::WebSocket(_)));
        std::env::remove_var("DEPYLER_MCP_TRANSPORT");
    }

    #[cfg(feature = "http")]
    #[test]
    fn test_from_env_http() {
        std::env::set_var("DEPYLER_MCP_TRANSPORT", "http://localhost:8080");
        let transport_type = TransportFactory::from_env().unwrap();
        assert!(matches!(transport_type, TransportType::Http(_)));
        std::env::remove_var("DEPYLER_MCP_TRANSPORT");
    }
}

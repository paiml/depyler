use anyhow::Result;
use pmcp::{StdioTransport, Transport};
use std::fmt::Debug;

#[cfg(feature = "websocket")]
use pmcp::WebSocketTransport;

#[cfg(feature = "http")]
use pmcp::HttpTransport;

#[derive(Debug, Clone)]
pub enum TransportType {
    Stdio,
    #[cfg(feature = "websocket")]
    WebSocket(String), // URL
    #[cfg(feature = "http")]
    Http(String), // URL
}

impl Default for TransportType {
    fn default() -> Self {
        TransportType::Stdio
    }
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
                let transport = WebSocketTransport::new(&url)?;
                Ok(Box::new(transport))
            }

            #[cfg(feature = "http")]
            TransportType::Http(url) => {
                let transport = HttpTransport::new(&url)?;
                Ok(Box::new(transport))
            }
        }
    }

    pub fn create_stdio() -> Result<StdioTransport> {
        Ok(StdioTransport::new())
    }

    #[cfg(feature = "websocket")]
    pub fn create_websocket(url: &str) -> Result<WebSocketTransport> {
        WebSocketTransport::new(url)
    }

    #[cfg(feature = "http")]
    pub fn create_http(url: &str) -> Result<HttpTransport> {
        HttpTransport::new(url)
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

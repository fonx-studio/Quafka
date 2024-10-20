//! Quafka - Kafka on HTTP/3 using QUIC

/// Quafka environment variables
pub mod variables {
    /// Cert path
    pub const CERT_PATH: &str = "CERT_PATH";
    /// Key path
    pub const KEY_PATH: &str = "KEY_PATH";
}

/// Quafka error types
pub mod error;

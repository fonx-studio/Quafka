use thiserror::Error;

/// Quafka Result
pub type Result<T> = std::result::Result<T, QuafkaError>;

#[derive(Error, Debug)]
/// Core error type for Quafka
pub enum QuafkaError {
    /// Missing application certificates from environment variable `CERT_PATH` or `KEY_PATH`
    #[error("Missing application certificates: {0}")]
    MissingCertificates(#[from] std::env::VarError),

    /// Quic invalid certificates error
    #[error("Invalid certificates")]
    InvalidCertificates,
    /// Quic io error: Failed to bind to port
    #[error("Failed to bind to host:port: {0}")]
    IO(#[from] std::io::Error),
    /// Quic start app error
    #[error("Quic start app error: {0}")]
    QuicStart(#[from] s2n_quic::provider::StartError),
}

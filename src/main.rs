//! Quafka - Kafka on HTTP/3 using QUIC

use std::env;

use quafka::{
    error::{QuafkaError, Result},
    variables::{CERT_PATH, KEY_PATH},
};

use s2n_quic::Server;
use tracing::{error, info, warn, Level};
use tracing_subscriber::{util::SubscriberInitExt, FmtSubscriber};

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    subscriber.init();
    info!("Starting Quafka");

    let cert_pem = read_to_string(&env::var(CERT_PATH)?);
    let key_pem = read_to_string(&env::var(KEY_PATH)?);
    info!("Certificates loaded");

    let mut server = Server::builder()
        .with_tls((cert_pem.as_str(), key_pem.as_str()))
        .map_err(|_| QuafkaError::InvalidCertificates)?
        .with_io("127.0.0.1:9092")?
        .start()?;

    while let Some(mut conn) = server.accept().await {
        info!("Connection accepted {:?}", conn.remote_addr());

        match conn.accept_receive_stream().await {
            Ok(Some(mut stream)) => {
                info!(
                    "Connection open from {:?}",
                    stream.connection().remote_addr()
                );
                warn!("{:?}", stream.receive().await);
            }
            Ok(None) => info!("Connection closed"),
            Err(err) => {
                error!("Connection error: {:?}", err);
                conn.close(1u32.into());
            }
        }
    }
    warn!("Quafka closed");
    Ok(())
}

fn read_to_string(path: &str) -> String {
    std::fs::read_to_string(path).expect("Unable to read file")
}

use bytes::Bytes;
use quafka::variables::CERT_PATH;
use s2n_quic::{client::Connect, Client};
use std::{env, error::Error, net::SocketAddr};

pub async fn message(input: &str) -> Result<(), Box<dyn Error>> {
    let cert_pem = read_to_string(&env::var(CERT_PATH)?);
    let client = Client::builder()
        .with_tls(cert_pem.as_str())?
        .with_io("0.0.0.0:0")?
        .start()?;

    let addr: SocketAddr = "127.0.0.1:9092".parse()?;
    let connect = Connect::new(addr).with_server_name("localhost");
    let mut connection = client.connect(connect).await?;

    connection.keep_alive(false)?;

    let mut send_stream = connection.open_send_stream().await?;
    send_stream
        .send(Bytes::copy_from_slice(input.as_bytes()))
        .await?;
    let _ = send_stream.flush().await;

    Ok(())
}

fn read_to_string(path: &str) -> String {
    std::fs::read_to_string(path).expect("Unable to read file")
}

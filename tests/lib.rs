pub mod client;

#[tokio::test]
async fn send_message() -> Result<(), Box<dyn std::error::Error>> {
    client::message("hello").await
}

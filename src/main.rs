use tokio;
use std::rc::Rc;

///////////////////////////////////////////////////////////////////////////////////////////////////
/// Main (Task handler)
#[tokio::main]
pub async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = client_construction();
    let cclient = client.clone();
    for _ in 0..=100 {
        tokio::task::spawn(async move{
            pingep(cclient).await;
        });
    }

    Ok(())
}

pub fn client_construction() -> reqwest::Client {
    let client = reqwest::Client::new();
    return client;
}

pub async fn pingep(
    client: reqwest::Client
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let body = client.get("https://www.supremenewyork.com/mobile_stock.json")
        .send()
        .await?;

    println!("PING: {:?}", body);
    Ok(())
}

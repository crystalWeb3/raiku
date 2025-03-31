use serde_json::json;
use serde_json::Value;
use reqwest::Client;

pub async fn fetch_transaction_details(signature: &str) -> Result<Value, reqwest::Error> {
    let url = "https://api.devnet.solana.com".to_string();
    let client = Client::new();
    let params = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransaction",
        "params": [signature, { "encoding": "json" }]
    });

    let res = client.post(&url)
        .json(&params)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

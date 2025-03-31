use serde_json::Value;
use crate::store::TRANSACTION_STORE;
use crate::utils::fetch_transaction_details;

pub async fn process_transaction(text: String) {
    if let Ok(parsed) = serde_json::from_str::<Value>(&text) {
        if let Some(logs) = parsed["params"]["result"]["value"]["logs"].as_array() {
            let signature = parsed["params"]["result"]["value"]["signature"].as_str().map(|s| s.to_string());
            let accounts = parsed["params"]["result"]["value"]["accounts"].as_array().cloned();

            if let Some(sig) = signature {
                if sig == "1111111111111111111111111111111111111111111111111111111111111111" {
                    return;
                }

                let is_sol_transfer = logs.iter().any(|log| {
                    let log_str = log.as_str().unwrap_or("");
                    log_str.contains("Transfer") && !log_str.contains("Token")
                });

                let program_id = parsed["params"]["result"]["value"]["programId"]
                    .as_str()
                    .unwrap_or("");
                
                if is_sol_transfer && program_id.is_empty() {
                    // println!("SOL Transfer detected: {}", sig);
                    
                    let sig_clone = sig.clone();

                    tokio::spawn(async move {
                        match fetch_transaction_details(&sig_clone).await {
                            Ok(transaction_data) => {
                                let account_keys = &transaction_data["result"]["transaction"]["message"]["accountKeys"];
                                let post_balances = &transaction_data["result"]["meta"]["postBalances"];

                                let sender = account_keys.get(0).and_then(|key| key.as_str()).unwrap_or("");
                                let receiver = account_keys.get(1).and_then(|key| key.as_str()).unwrap_or("");
                                let sender_balance = post_balances.get(0).and_then(|balance| balance.as_i64()).unwrap_or(0);
                                let receiver_balance = post_balances.get(1).and_then(|balance| balance.as_i64()).unwrap_or(0);

                                if sender != "" && sender_balance != receiver_balance {
                                    let amount = sender_balance - receiver_balance;
                                    let timestamp = transaction_data["result"]["blockTime"].as_i64().unwrap_or(0);

                                    println!("Data: {} {} {} {} {}",sig,  sender, receiver, amount, timestamp);
                                    
                                    let transaction_info = std::collections::HashMap::from([
                                        ("sender".to_string(), sender.to_string()),
                                        ("receiver".to_string(), receiver.to_string()),
                                        ("amount".to_string(), amount.to_string()),
                                        ("timestamp".to_string(), timestamp.to_string()),
                                    ]);

                                    let mut store = TRANSACTION_STORE.lock().await;
                                    store.insert(sig_clone, transaction_info);
                                }
                            }
                            Err(e) => eprintln!("Failed to fetch transaction details: {}", e),
                        }
                    });
                }
            }

            if let Some(accs) = accounts {
                println!("Accounts involved: {:?}", accs);
            }
        }
    }
}

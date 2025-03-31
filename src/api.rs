use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
    serve, 
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::collections::HashMap;
use serde_json::json;
use crate::store::TRANSACTION_STORE; // Ensure this global store is properly defined
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct TransactionQuery {
    id: Option<String>,
    day: Option<String>,
}

async fn fetch_transactions(query: Query<TransactionQuery>) -> Result<Json<serde_json::Value>, StatusCode> {
    let store = TRANSACTION_STORE.lock().await;

    // If an ID is provided, return a single transaction    
    if let Some(transaction_id) = &query.id {
        if let Some(transaction) = store.get(transaction_id) {
            return Ok(Json(json!(transaction)));
        }
    }

    

    // If filtering by day, check timestamps
    if let Some(day) = &query.day {
        
        let day_timestamp: i64 = day.parse().unwrap_or(0);
        println!("Fetching Date {}", day_timestamp);
        let filtered_transactions: HashMap<_, _> = store
            .iter()
            .filter(|(_, details)| {
                details.get("timestamp")
                    .and_then(|t| t.parse::<i64>().ok())
                    .map(|t| t >= day_timestamp && t < day_timestamp + 86400)
                    .unwrap_or(false)
            })
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        if !filtered_transactions.is_empty() {
            return Ok(Json(json!(filtered_transactions)));
        }
    }

    // If no data found, return 404
    Err(StatusCode::NOT_FOUND)
}

async fn okay_response() -> Json<&'static str> {
    Json("Server is runniing well.")
}

pub async fn start_api() {
    let app = Router::new()
        .route("/transactions", get(fetch_transactions))
        .route("/", get(okay_response));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Attempting to bind to {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap_or_else(|e| {
        eprintln!("Failed to bind to {}: {}", addr, e);
        std::process::exit(1);
    });

    println!("🚀 API running on http://{}", addr);

    serve(listener, app.into_make_service())
        .await
        .unwrap();
}

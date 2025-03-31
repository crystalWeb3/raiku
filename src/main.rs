mod websocket;
mod transactions;
mod utils;
mod store;
mod api;

#[tokio::main]
async fn main() {
    tokio::join!(
        websocket::start_websocket(),
        api::start_api()
    );
}

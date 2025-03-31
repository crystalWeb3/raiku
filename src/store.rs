use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;

pub static TRANSACTION_STORE: Lazy<Arc<Mutex<HashMap<String, HashMap<String, String>>>>>
    = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

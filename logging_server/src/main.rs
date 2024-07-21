use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct LogMessage {
    level: String,
    message: String,
}

type LogStore = Arc<Mutex<VecDeque<LogMessage>>>;

#[tokio::main]
async fn main() {
    let log_store = LogStore::default();

    let log_store_filter = warp::any().map(move || log_store.clone());

    let log_route = warp::post()
        .and(warp::path("log"))
        .and(warp::body::json())
        .and(log_store_filter.clone())
        .map(|log_message: LogMessage, log_store: LogStore| {
            let mut store = log_store.lock().unwrap();
            store.push_back(log_message);
            warp::reply::json(&"Log received")
        });

    let get_logs_route = warp::get()
        .and(warp::path("logs"))
        .and(log_store_filter.clone())
        .map(|log_store: LogStore| {
            let store = log_store.lock().unwrap();
            let logs: Vec<_> = store.iter().cloned().collect();
            warp::reply::json(&logs)
        });

    let routes = log_route.or(get_logs_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

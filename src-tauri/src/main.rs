// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{vec, collections::HashMap};

use serde::{Deserialize, Serialize};
use tauri::{State, Manager, AppHandle};
use tokio::sync::Mutex;

#[tauri::command]
async fn greet(name: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    client
        .get(name)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_requests(reqs: State<'_, Storage>) -> Result<HashMap<String, Request>, ()> {
    Ok(reqs.saved_requests.lock().await.clone())
}

#[tauri::command]
async fn set_request(state: State<'_, Storage>, app: AppHandle, url: String, name: String) -> Result<String, ()> {
    let mut reqs = state.saved_requests.lock().await;
    let mut id = name.clone();
    let mut index = 1;
    while reqs.contains_key(&id) {
        id = format!("{name}-{index}");
        index += 1;
    };
    reqs.insert(id.clone(), Request {
        id: id.clone(),
        name,
        url,
        body: None,
        description: None,
        headers: vec![],
        method: String::from("GET"),
    });
    app.emit_all("update-requests", ()).map_err(|_| ())?;
    Ok(id)
}
fn main() {
    tauri::Builder::default()
        .manage(Storage {
            saved_requests: Mutex::new(HashMap::default()),
        })
        .invoke_handler(tauri::generate_handler![greet, set_request, get_requests])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct Storage {
    saved_requests: Mutex<HashMap<String, Request>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Request {
    id: String,
    name: String,
    description: Option<String>,
    url: String,
    method: String,
    headers: Vec<Header>,
    body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Header {
    key: String,
    value: String,
}

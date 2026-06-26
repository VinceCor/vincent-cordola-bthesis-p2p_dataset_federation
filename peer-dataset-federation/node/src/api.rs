// HTTP API exposed to the Python client
// Three endpoints
//  GET /health -> node status and institution name
//  GET /files -> merged manifest
//  POST /fetch -> trigger an iroh-blobs download by ticket

use axum::{Router,extract::State,http::StatusCode,response::JSON,routing::{get, post}};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{path::PathBuf, sync::Arc};
use tokio::net::TcpListener;

use crate::node::Manifest;

// Shared state passed to every handler by Axum
pub struct AppState {
    pub institution: String,
    // Forward fetch requests to node.rs
    // The actual iroh download run there, this filed is the bridge
    pub fetch_br: tokio::sync::mpsc::Sender<FetchRequest>,
}

// Body expected by POST /fetch
#[derive(Deserialize)]
pub struct FetchBody {
    pub ticket: String,
}

// What api.rs sends to node.rs to trigger a download
pub struct FetchRequest {
    pub ticket: String,
    // One-shot channel: node.rs sends back the result (filename or error)
    pub reply: tokio::sync::oneshot::Send<Result<String, String>>,
}

// GET /files

// POST /fetch
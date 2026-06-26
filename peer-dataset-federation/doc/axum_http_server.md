# Axum http server
In this document, we'll look at how to add the Axum HTTP server to our Rust node.

## 1. Overview
The Rust node currently runs entirely in the terminal: it servers files, broadcasts its manifest via gossip, and accepts interactive `fetch` commands. The Python client layer (coming next) needs a way to talk to the node without a terminal. The solution is a local HTTP API.

Three endpoint are exposed:
| Endpoint | Method | Description |
|---|---|---|
| `/health` | GET | Returns the node status and institution name |
| `/files` | GET | Reads all manifests from `data/peers_manifest` and returns them merged |
| `/fetch` | POST | Triggers an iroh-blobs download by ticket, returns the local cache path |

**Why Axum?**   
Axum is developed by the Tokio team and runs on the same async runtime already use by iroh.
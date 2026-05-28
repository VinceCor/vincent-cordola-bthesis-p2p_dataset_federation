# Rust MVP installation
This document provides the basis for creating an initial MVP that enables the exchange of Parquet files between two terminals.

## Table of Contents

## 1. Setup Rust project

### 1.1 Install Rust and Cargo
> This comes from [Rust and Cargo installation](https://doc.rust-lang.org/cargo/getting-started/installation.html)

On linux and macOS systems:
```bash
curl https://sh.rustup.rs -sSf | sh 
```

### 1.2 Project setup
> The following codes can be retrieved directly from [What is iroh?](https://docs.iroh.computer/what-is-iroh) and [iroh quickstart](https://docs.iroh.computer/quickstart)   
   

First, initialize a new Rust project:
```bash
cargo init rust-mvp
cd rust-mvp
cargo run
```
This should print `Hello, world!`

## 2. Join an ad hoc iroh network
In this section, I'll show you how to connect two endpoints.

### 2.1 Dependencies
Add dependencies (this can be done directory in Cargo.toml)

Cargo.toml:
```
[package]
name = "rust-mvp"
version = "0.1.0"
edition = "2024"

[dependencies]
iroh = "0.98.2"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
n0-error = "0.1"
tracing-subscriber = "0.3"
```
| Dependencies | Role |
|--------------|------|
| iroh | [Modular networking stack for direct p2p connections between devices](https://www.iroh.computer/)| 
| tokio | [Asynchronous Rust runtime](https://tokio.rs/) |
| anyhow | [Simplifies error handling](https://google.github.io/comprehensive-rust/error-handling/anyhow.html) |
| n0-error | [Iroh compatible error handling](https://www.iroh.computer/blog/iroh-0-95-0-new-relay) |
| tracing-subscriber | [Iroh uses this to display internal logs in the terminal](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/) |

### 2.2 Structure

```
src/
    main.rs
    node.rs
```
`main.rs` simply reads what the user wants to do and calls appropriate function. All the network complexity is encapsulated in `node.rs`.


### 2.3 main.rs
> References: [Creating an Endpoint](https://docs.iroh.computer/connecting/creating-endpoint), [iroh example listen.rs](https://github.com/n0-computer/iroh/blob/main/iroh/examples/listen.rs) and [iroh example connect.rs](https://github.com/n0-computer/iroh/blob/main/iroh/examples/connect.rs)

```Rust
mod node;
use iroh::{EndpointAddr, PublicKey};
use std::{env};

// Create a Tokio runtime and uses it to run `main` as an async function
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initializing log
    tracing_subscriber::fmt::init();

    // Reading the arguments and dispatching
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("listen") => {
            node::listen().await?;
        }
        Some("connect") => {
            // Preparing the address in connect mode
            let id_str = args.get(2).expect("Usage: connect <EndpointId>");
            let public_key: PublicKey = id_str.parse()?;
            let addr = EndpointAddr::new(public_key);
            node::connect(addr).await?;
        }

        _ => {
            eprintln!("Usage:");
            eprintln!("cargo run -- listen");
            eprintln!("cargo run -- connect <EndpointId>")
        }
    }
    Ok(())
}
```

### 2.4 node.rs

```Rust
/* 
`node.rs` exposes two public functions and a constant. The ALPN constant
is shared between the two functions, it is the protocol identifier
that allows the two peers to recognize each other.

References: 
    Creating an Endpoint: https://docs.iroh.computer/connecting/creating-endpoint 
    iroh example listen.rs: https://github.com/n0-computer/iroh/blob/main/iroh/examples/listen.rs
    iroh example connect.rs: https://github.com/n0-computer/iroh/blob/main/iroh/examples/connect.rs
    iroh sendme example: https://github.com/n0-computer/sendme
*/
use iroh::{Endpoint, EndpointAddr, endpoint::presets};
use n0_error::{Result, StdResultExt};

// Example ALPN use to communicate over the `Endpoint`. Taken from https://github.com/n0-computer/iroh/blob/main/iroh/examples/listen.rs
pub const ALPN: &[u8] = b"p2p-parquet/0";

// Listen mode, waiting for connections
pub async fn listen() -> Result<()> {
    // Creating the endpoint
    // Configure with the default settings: relay servers enabled, DNS discovery enabled
    let endpoint = Endpoint::builder(presets::N0)
        // Set the ALPN protocols this endpoint will accept 
        .alpns(vec![ALPN.to_vec()])
        .bind()
        .await?;

    // `endpoint.add().id` is the local node's public key. This is the value that the user of terminal B will need to copy and paste
    let addr = endpoint.addr();
    println!("Endpoint ID : {}", addr.id);
    println!("Full address : {addr:?}");
    println!("Waiting for connection");

    // Accept loop, `endpoint.accept().await blocks until the incoming connection`
    while let Some(incoming) = endpoint.accept().await {
        let conn = incoming.await.std_context("Incoming connexion error")?;
        let remote = conn.remote_id();
        println!("Connection received from : {remote}");
        conn.closed().await;
    }

    Ok(())
}

// Connect mode, connect to a peer
pub async fn connect(addr: EndpointAddr) -> Result<()> {
    // Creating the endpoint
    let endpoint = Endpoint::bind(presets::N0).await?;
    println!("Local endpoint ID: {}", endpoint.addr().id);

    // Establishing the connection
    let conn = endpoint.connect(addr, ALPN).await?;
    println!("Connect to : {}", conn.remote_id());

    // Connexion closed
    conn.close(0u32.into(),b"Connection close!");
    endpoint.close().await;

    Ok(())
}
```


![iroh network listen](media/iroh_network_listen.png)

![iroh network connect](media/iroh_network_connect.png)






## 3. todo
> iroh-blobs: https://docs.iroh.computer/protocols/blobs    
iroh-blobs example: transfer.rs https://github.com/n0-computer/iroh-blobs/blob/main/examples/transfer.rs    
sendme (iroh blobs file transfer example) https://github.com/n0-computer/sendme





## References
##### R1 | [Rust and Cargo installation](https://doc.rust-lang.org/cargo/getting-started/installation.html)
##### R2 | [What is iroh?](https://docs.iroh.computer/what-is-iroh)
##### R3 | [iroh quickstart](https://docs.iroh.computer/quickstart)
##### R4 | [Creating an Endpoint](https://docs.iroh.computer/connecting/creating-endpoint)
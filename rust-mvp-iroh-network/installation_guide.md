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
These sections are separated to make the program easier to read and modfiy the project in the future.


### 2.3 main.rs
> References: [Creating an Endpoint](https://docs.iroh.computer/connecting/creating-endpoint), [iroh example listen.rs](https://github.com/n0-computer/iroh/blob/main/iroh/examples/listen.rs), [iroh example connect.rs](https://github.com/n0-computer/iroh/blob/main/iroh/examples/connect.rs) and [tokio hello tutorial](https://tokio.rs/tokio/tutorial/hello-tokio)

Whit this code, we'll do three things for now: initialize the logs, read the CLI arguments, and call the appropriate function in `node.rs`

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
**`#[tokio::main]`and `async fn main`**     
Rust does not natively support async functions as entry points. The `#[tokio::main]` macro solvie this: it creates a Tokio runtime and uses it to execute `main` as an async functions. All of the project's `async/await` code runs within this runtime 

**Logs initialization**
```Rust
tracing_subscriber::fmt::init();
```
Iroh uses the `tracing` framework for its internal logs. Without this line, no Iroh logs will be visible.

**Read the argument and dispatch**
```Rust
let args: Vec<String> = env::args().collect();
match args.get(1).map(String::as_str) {...}
```
`env::args()` returns the arguments passed after `cargo run --`. The `match` redirects to `listen` or `connect`. The `_` displays instructions on how to use the command if the argument is unknown.

**Address preparation in mode `connect`**
> References: [Struct EndpointAddr](https://docs.rs/iroh/latest/iroh/struct.EndpointAddr.html) and [iroh endpoint](https://docs.iroh.computer/concepts/endpoints)
```Rust
let public_key: PublicKey = id_str.parse()?;
let addr = EndpointAddr::new(public_key);
```
The user provides the listener's `EndpointId` (display in their terminal) as a string. These two lines convert it into an `EndpointAddr` that iroh can use to establish the connection. 

### 2.4 node.rs
`node.rs` exposes two public functions and a constant. The ALPN constant is hared between the two functions, it is the protocol identifier that allows the two peers to recognize each other.

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
    iroh protocols: https://docs.iroh.computer/concepts/protocols
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

**The ALPN constant**
> references: [iroh protocols](https://docs.iroh.computer/concepts/protocols) and [wikipedia ALPN](https://en.wikipedia.org/wiki/Application-Layer_Protocol_Negotiation)
```Rust
pub const ALPN: &[u8] = b"p2p-parquet/0";
```
ALPN(Application-Layer Protocol Negotiation) is an identifier exchanged during the QUIC handshake. If the listener and the connector do not have the same ALPN, the connection is rejected. This is the mechanism by which iroh determines which application protocol will run over the connection.

**`listen` mode, waiting for connections**
```Rust
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
```
> References: [iroh docs, endpoint accept](https://docs.rs/iroh/latest/iroh/endpoint/struct.Endpoint.html#method.accept) and [iroh::endpoint Struct connexion](https://docs.rs/iroh/latest/iroh/endpoint/struct.Connection.html)

**Creating the endpoint**   
`Endpoint::builder(presets::N0)` create an endpoint configured with the default settings: relay servers enabled, DNS discovery enabled. `.alpns(vec![ALPN.to_vec()])` is required on the listener side. Without it, iroh rejects all incoming connections.


**Displaying the address**  
`endpoint.addr().id` is the local node's `PublicKey`. This is the value that the user of terminal B must use. It uniquely and permanently identifies the node on the iroh network.


**Accept loop**     
`endpoint.accept().await` blocks until the next incoming connection. The `while let Some` loop runs indefinitely, the `None` value is only returned if the endpoint is explicitly closed. `incoming.await` completes the TLS/QUIC handshake: at this point, the connection is encrypted and authenticated. `conn.remote_id()` returns the remote peer's `PublicKey`.


**`connect` mode, connect to a peer**
```Rust
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

**Creating the endpoint**   
On the connector side, there's no need to decale ALPN, this is only required to accept incoming connections.

**Establishing the connection**
`endpoint.connect(addr, ALPN).await?` is the central call. Iroh handles all network complexity, it first attempts a direct connection, uses hole punching if necessary, and falls back to the relay server if no direct connection is possible. The `ALPN` passed here must match exactly the one declared by the listener

**Closing the connection**
> References: [QUIC RFC 9000](https://www.rfc-editor.org/info/rfc9000/#section-10.2)

`conn.close(0u32.into(),b"Connection close!")` sends a QUIC CONNECTION_CLOSE frame. `endpoint.close().await` is asynchronous and waits for all pending messages to be sent before closing the UDP socket. WIthout this call, the closure might be truncated.



### 2.5 Result
In this section, we will attempt to establish a connection between two endpoint. To do this, we will use two terminals.

First, we launch our `listen` function `cargo run -- listen`. We can see our `EndpointId`and `PublicKey` (highlighted in yellow), and later we'll also be able to see the other peer's `EndpointId`.
![iroh network listen](media/iroh_network_listen.png)

Once our `listen` is up and running, we can run our `connect` function in the second terminal. You'll need to specify the `PublicKey`retrieved from our first endpoint `cargo run -- connect <PublicKey>`. When you run the command, you'll see in the first terminal that the connection has been successfully established.

![iroh network connect](media/iroh_network_connect.png)

In both terminals, we can see that each `EndpointId` is present. This confirms that our two endpoints were able to communicate successfully.


## 3. Advertise and fetch Parquet files

### 3.1 main.rs change
```Rust
// file transfer
Some("listen-blobs") => {
    // Process all .parquet files in the data/ directory. Displaying one ticket per file
    // and then waits indefinitely for incoming connections
    node::listen_blobs().await?;
}

Some("fetch-blobs") => {
    let tickets: Vec<String> = args[2..].to_vec();
    if tickets.is_empty() {
        eprintln!("Usage: cargo run -- fetch-blobs <ticket1> ...");
        std::process::exit(1);
    }
    node::fetch_blobs(tickets).await?;
}
```

### 3.2 node.rs listen_blobs()
```Rust
// listen_blobs: Process all .parquet files in the data/ directory. Displaying one ticket per file...
pub async fn listen_blobs() -> Result<()> {
    // Create the iroh endpoint
    let endpoint = Endpoint::bind(presets::N0).await?;

    println!("Listener: {:?}", endpoint.addr());

    // Create the in-memory store and the blob protocol
    let store = MemStore::new();
    let blobs = BlobsProtocol::new(&store, None);

    // Hash all .parquet files in data/, for each file, iroh-blobs calculates its BLAKE3 hash
    let data_dir = PathBuf::from("data");
    let mut entries = tokio::fs::read_dir(&data_dir)
        .await
        .std_context("Unable to read data/ folder")?;

    println!("File hashing in data/");
    while let Some(entry) = entries.next_entry().await.std_context("Error reading entry")? {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("parquet") {
            let abs_path = path.canonicalize().std_context("Absolute path not found")?;
            let filename = path.file_name().unwrap().to_string_lossy().to_string();

            // add_path hashes the file and returns a tag (hash + format)
            // The "tag" prevents the store's garbage collector from deleting the blob
            let tag = store.blobs().add_path(abs_path).await
                .std_context("Error during hashing")?;

            // BlobTicket -> Blake 3 hash of the file + listener's EndpointId
            // The only information the connector needs to fetch
            let ticket = BlobTicket::new(endpoint.id().into(), tag.hash, tag.format);
            println!(" [{}] ticket: {}", filename,ticket);

        }
    }

    // Start the Router
    // The router replaces the loop `while let Some(incoming)` from fn listen()
    // It accepts incoming connections and routes them to iroh-blobs via ALPN
    let router = Router::builder(endpoint)
        .accept(iroh_blobs::ALPN, blobs)
        .spawn();

    println!("Waiting for connections");

    // Wait for Ctrl+C for to exit properly
    tokio::signal::ctrl_c().await.std_context("Signal Ctrl+C")?;
    router.shutdown().await.std_context("Error shutdown")?;

    Ok(())
}
```

### 3.3 node.rs fetch_blobs()
```Rust
// fetch_blobs: download the files associated with the tickets to cache/
pub async fn fetch_blobs(raw_tickets: Vec<String>) -> Result <()> {
    // Create the endpoint and store on the connector side
    let endpoint = Endpoint::bind(presets::N0).await?;

    println!("Connector: {:?}", endpoint.addr());

    let store = MemStore::new();

    // Create the cache/ folder if it doesn't exist
    let cache_dir = PathBuf::from("cache");
    tokio::fs::create_dir_all(&cache_dir)
        .await
        .std_context("Unable to create cache/")?;

    // Parse tickets and prepare downloads
    // The downloader coordinates requests to one or more peers.
    // Reusing the same downloader for multiple files is more efficient because iroh can reuse the already-open QUIC connection.
    let downloader = store.downloader(&endpoint);

    let mut ticket_list: Vec<BlobTicket> = Vec::new();

    for raw in &raw_tickets {
        let ticket: BlobTicket = raw.parse().std_context("Invalid ticket")?;
        ticket_list.push(ticket);
    }

    println!("Start downloads ({} files)", ticket_list.len());

    for (i, ticket) in ticket_list.iter().enumerate() {
        downloader
            .download(ticket.hash(), Some(ticket.addr().id))
            .await
            .std_context("Error during download")?;

        // File name: truncated hash + .parquet extension
        let filename = format!("{}.parquet", &ticket.hash().to_string()[..16]);

        // This section (dest) was created using Claude ai
        let dest = cache_dir.canonicalize()
            .std_context("Cannot resolve cache/ to absolute path")?
            .join(&filename);

        // export() copies the blob from MemStore to the file system
        store.blobs().export(ticket.hash(), dest.clone()).await
            .std_context("Error during export")?;

        println!(" [{}] cache/{}", i + 1, filename);
    }

    println!("All files are in cache/");

    endpoint.close().await;


    Ok(())
}
```


## x. todo
> iroh-blobs: https://docs.iroh.computer/protocols/blobs    
iroh-blobs example: transfer.rs https://github.com/n0-computer/iroh-blobs/blob/main/examples/transfer.rs    
sendme (iroh blobs file transfer example) https://github.com/n0-computer/sendme



## References
##### R1 | [Rust and Cargo installation](https://doc.rust-lang.org/cargo/getting-started/installation.html)
##### R2 | [What is iroh?](https://docs.iroh.computer/what-is-iroh)
##### R3 | [iroh quickstart](https://docs.iroh.computer/quickstart)
##### R4 | [Creating an Endpoint](https://docs.iroh.computer/connecting/creating-endpoint)
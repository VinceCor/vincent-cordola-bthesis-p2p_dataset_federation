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
> This version of Cargo allows you to run all the code included in the iroh_setup_guide. For the project itself, please refer directly to the project's Cargo.

Add dependencies (this can be done directory in Cargo.toml).

Cargo.toml:
```
[package]
name = "rust-mvp"
version = "0.1.0"
edition = "2024"

[dependencies]
iroh = "0.98.2"
iroh-blobs = "0.100.0"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
n0-error = "0.1"
tracing-subscriber = "0.3"
futures = "0.3"
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
> References: [iroh-blobs](https://docs.iroh.computer/protocols/blobs), [transfer.rs](https://github.com/n0-computer/iroh-blobs/blob/main/examples/transfer.rs), [iroh Sendme](https://github.com/n0-computer/sendme)
### 3.1 main.rs change
Two new CLI modes are added to `main.rs`,
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

`listen-blobs`
```Rust
Some("listen-blobs") => {
    node::listen_blobs().await?;
}
```
No arguments are needed. The listener scans /data itself and generates one ticket per file.


`fetch-blobs`
```Rust
Some("fetch-blobs") => {
    let tickets: Vec<String> = args[2..].to_vec();
    if tickets.is_empty() {
        eprintln!("Usage: cargo run -- fetch-blobs <ticket1> ...");
        std::process::exit(1);
    }
    node::fetch_blobs(tickets).await?;
}
```
`args[2..]` slices the argument list starting from the third element (index 2), skipping binary name (index 0) and the `fetch-blobs` command.   



### 3.2 node.rs listen_blobs()
> References: [iroh Tickets](https://docs.iroh.computer/concepts/tickets),[Blob ticket](https://docs.rs/iroh-blobs/latest/iroh_blobs/ticket/struct.BlobTicket.html), [Iroh Router](https://docs.rs/iroh/latest/iroh/protocol/struct.Router.html)
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
    // The `while let Some` section whas created using Claude chatbot
    while let Some(entry) = entries.next_entry().await.std_context("Error reading entry")? {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("parquet") {
            // `canonicalize` resolves the path to an absolute, canonical form
            let abs_path = path.canonicalize().std_context("Absolute path not found")?;
            let filename = path.file_name().unwrap().to_string_lossy().to_string();

            // add_path hashes the file and returns a tag (hash + format), requires an absolute path
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
**Creating the endpoint**
```Rust
let endpoint = Endpoint::bind(presets::N0).await?;
```
Unlike `listen()` in section 2, there is no `.alpns(vec![...])` call here. This is because the ALPN is now declared at the `Router` level.

**Creating the store and the blobs protocol**
```Rust
let store = MemStore::new();
let blobs = BlobsProtocol::new(&store, None);
```
`MemStore`is an in-memory blob store. It holds the file data and the BLAKE3 metadata. The store lives for the duration of the program.  
`BlobsProtocol::new(&store, None)` wraps the store into a protocol handler. This is the object that will be given to the `Router` to handle incoming blob requests.

**Scanning and hashing the files**
```Rust
let data_dir = PathBuf::from("data");
let mut entries = tokio::fs::read_dir(&data_dir)
    .await
    .std_context("Unable to read data/ folder")?;
```
`PathBuf::from("data")` creates a relative path.    
`tokio::fs::read_dir` opens a directory stream without blocking the Tokio runtimes.

```Rust
if path.extension().and_then(|e| e.to_str()) == Some("parquet") 
```
The `if` filter uses `path.extension()` which returns `Option<&OsStr>`. `and_then(|e| e.to_str())` converts it to `Option<&str>`, and comparing to `Some("parquet")` ensures only `.parquet` files are processed. Any other file in `data/` is skipped.

**Hasing a file and producing its ticket**
```Rust
let tag = store.blobs().add_path(abs_path).await
    .std_context("Error during hashing")?;
```
`add_path` reads the file, computes its BLAKE3 hash, store the outboard metadata in the `MemStore`, and returns a `Tag`.The `Tag`contains two fileds: `tag.hash` (the 32-byte BLAKE3 root hash that uniquely identifies the file's content) and tag.format (`raw` for a single blob).   
Keeping the `tag` alive in the store is important: iroh-blobs has a garbage collector, and a blob with no live tag reference is eligible for deletion. 

```Rust
let ticket = BlobTicket::new(endpoint.id().into(), tag.hash, tag.format);
```
`BlobTicket` encodes three things into a single shareable string: endpoint address (node ID, relay URL, and direct addresses) plus optional application-specifif data like a document ID or blob hash. References: [iroh tickets](https://docs.iroh.computer/concepts/tickets)


**Starting the Router**
```Rust
let router = Router::builder(endpoint)
    .accept(iroh_blobs::ALPN, blobs)
    .spawn();
```
The `Router` replaces the manual `while let Some(incoming) = endpoint.accept().await` from section 2's `listen()`. It runs the accept loop in the background and routes each incoming connection to the coorect protocol handler based on the ALPN string.  
`.spawn()` launches the router as a background Tokio task.

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

    // Downloading and exporting files
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

**Creating the endpoint and store**
```Rust
let endpoint = Endpoint::bind(presets::N0).await?;
let store = MemStore::new();
```
The connector creates its own independent endpoint and its own `MemStore`. The store here acts as a download buffer: blobs are received and verified into memory first, then exported to disk in a second step.

**Downloader**
```Rust
let downloader = store.downloader(&endpoint);
```
`store.downloader(&endpoint)` create a `Downloader` bound to both the store and the endpoint. The `Downloader` is the object reponsible for opening iroh connections to providers and pulling blobs. Reusing the same `Download` instance acrross multiple downloads allows iroh to reuse the existing QUIC connection for the same peer, avoiding the overhead of a new handshake per file.

### 3.3 Result
For this test, we will also use two terminals, just as in the previous test.

**Files**   
To do this, I first created this file structure:
```
rust-mvp
    cache/
    data/
        sample.parquet
        sample2.parquet
        sample3.parquet
    src/
        main.rs
        node.rs
```

**Listener**    
First, we'll start the listener taht will make our Parquet files available as tickets.  
`cargo run -- listen-blobs`
![iroh listen-blobs](media/iroh_listen-blobs.png)

**Connector**   
Now we can launch our connector, which will be able to retrieve our tickets. To do this, we run our `fetch-blobs` command followed by the files we want to retrieve. `cargo run -- fetch-blobs <ticket1> <ticket2> ...`
![iroh fetch-blobs](media/iroh_fetch-blobs.png)

This is an early version, so we have to enter the tickets manually. We'll be able to improve this in the future using iroh-gossip, which will allow us to automatically discover peers.     
Another addition would be to have a file-naming convetion that allows us to see which file corresponds to which hash.

## 4. Listen and Connector in parallel
```Rust
// peer: single process that simultaneously serves local files and accepts interactive fetch commands.
// Commands:
//      - fetch <ticket> | download the file identified by the ticket
//      - quit           | shutdown
pub async fn peer() -> Result<()> {
    // Shared endpoint and store
    let endpoint = Endpoint::bind(presets::N0).await?;
    println!("Peer: {:?}", endpoint.addr());

    let store = MemStore::new();

    // hash local files
    let blobs = BlobsProtocol::new(&store, None);

    let data_dir = PathBuf::from("data");
    let mut entries = tokio::fs::read_dir(&data_dir)
        .await
        .std_context("Unable to read data/ folder")?;

    println!("File hashing in data/");

    // The `while let Some` section whas created with the help Claude chatbot
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
    // .spawn() launches the accept loop as a background Tokio task
    // The Router stays alive as long as this variable is in scope
    let router = Router::builder(endpoint.clone())
        .accept(iroh_blobs::ALPN, blobs)
        .spawn();

    // Prepare cache/ and the downloader once, reused for every fetch command
    let cache_dir = PathBuf::from("cache");
    tokio::fs::create_dir_all(&cache_dir)
        .await
        .std_context("Unable to crate cache/")?;

    let downloader = store.downloader(&endpoint);

    // Interactive command loop
    // tokio::io::BufReader wraps stdin so that read_line().await yields control back to the Tokio runtime while waiting for input
    // instead of blocking the thread. This is what allow the Router to keep serving incoming connections
    println!("Router started. Type 'fetch <ticket>' or 'quit'");

    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut lines = tokio::io::AsyncBufReadExt::lines(stdin);

    // lines.next_line().await block asynchronously until the user presses Enter
    // The loop exits when next_line() returns Non (stdin closed) or on "quit"
    // This section was created with the help of Claude chatbot
    while let Some(line) = lines.next_line().await.std_context("Error reading stdin")? {
        let line = line.trim().to_string();

        match line.split_once(' ') {
            Some(("fetch", raw_ticket)) => {
                let ticket: BlobTicket = match raw_ticket.trim().parse() {
                    Ok(t) => t,
                    Err(e) => {
                        println!("Invalid ticket: {e}");
                        print!("> ");
                        continue;
                    }
                };

                // Download the blob from the provider identified in the ticket
                match downloader
                    .download(ticket.hash(),Some(ticket.addr().id))
                    .await {
                        Ok(_) => {
                            //
                            let filename = format!("{}.parquet", &ticket.hash().to_string()[..16]);
                            let dest = cache_dir
                                .canonicalize()
                                .std_context("Cannot resolve cache/")?
                                .join(&filename);
                            match store.blobs().export(ticket.hash(), dest).await {
                                Ok(_) => println!("-> cache/{filename}"),
                                Err(e) => println!("Export error: {e}"),
                            }
                        }
                        Err(e) => println!("Download error: {e}"),
                }
            }
            // quit
            _ if line == "quit" => {
                break;
            }

            // unkown command
            _ if line.is_empty() => {}
            _ => println!("Unkown command")
        }

        print!("> ")
    }


    // Shutdown
    router.shutdown().await.std_context("Error shutdown")?;
    endpoint.close().await;

    Ok(())
}
```

### 4.1 main.rs peer
```rust
Some("peer") => {
    node::peer().await?;
}
```
`peer` does not require CLI arguments, tickets are entered interactively.

### 4.2 node.rs
**Problem to be solved**    
When a program waits for a keyboard input, it "blocks". In standard Rust, `stdin().read_line()` would block the entire thread, which would kill the Router running in the background.   
Tokio handles this with its asynchronous versions of I/O operations.

```rust
let stdin = tokio::io::BufReader::new(tokio::io::stdin());
```
`tokio::io::stdin()` is the asynchronous version of the standard keyboard input. `BufReader` wraps it so that it can read line by line rather than character by character.

```rust
while let Some(line) = lines.next_line().await... {...}
```
`next_line().await` waits for the next line, but in the meantime it lets the other tasks run. The `.await` allows the Router to send files even when we haven't typed anything.

## Validation on Docker Compose network 
> Objective: Validate peer mode on a network of 3 to 5 isolated Docker containers using manual ticket exchange.


## References

##### R1 | [Rust and Cargo installation](https://doc.rust-lang.org/cargo/getting-started/installation.html)
##### R2 | [What is iroh?](https://docs.iroh.computer/what-is-iroh)
##### R3 | [iroh quickstart](https://docs.iroh.computer/quickstart)
##### R4 | [Creating an Endpoint](https://docs.iroh.computer/connecting/creating-endpoint)
##### R5 | [iroh example listen.rs](https://github.com/n0-computer/iroh/blob/main/iroh/examples/listen.rs)
##### R6 | [iroh example connect.rs](https://github.com/n0-computer/iroh/blob/main/iroh/examples/connect.rs)
##### R7 | [tokio hello tutorial](https://tokio.rs/tokio/tutorial/hello-tokio)
##### R8 | [Struct EndpointAddr](https://docs.rs/iroh/latest/iroh/struct.EndpointAddr.html)
##### R9 | [iroh endpoint](https://docs.iroh.computer/concepts/endpoints)
##### R10 | [iroh protocols](https://docs.iroh.computer/concepts/protocols)
##### R11 | [wikipedia ALPN](https://en.wikipedia.org/wiki/Application-Layer_Protocol_Negotiation)
##### R12 | [iroh docs, endpoint accept](https://docs.rs/iroh/latest/iroh/endpoint/struct.Endpoint.html#method.accept)
##### R13 | [iroh::endpoint Struct Connection](https://docs.rs/iroh/latest/iroh/endpoint/struct.Connection.html)
##### R14 | [QUIC RFC 9000](https://www.rfc-editor.org/info/rfc9000/#section-10.2)
##### R15 | [iroh-blobs](https://docs.iroh.computer/protocols/blobs)
##### R16 | [transfer.rs](https://github.com/n0-computer/iroh-blobs/blob/main/examples/transfer.rs)
##### R17 | [iroh Sendme](https://github.com/n0-computer/sendme)
##### R18 | [iroh Tickets](https://docs.iroh.computer/concepts/tickets)
##### R19 | [Blob ticket](https://docs.rs/iroh-blobs/latest/iroh_blobs/ticket/struct.BlobTicket.html)
##### R20 | [Iroh Router](https://docs.rs/iroh/latest/iroh/protocol/struct.Router.html)
##### R21 | [iroh.computer](https://www.iroh.computer/)
##### R22 | [tokio.rs](https://tokio.rs/)
##### R23 | [anyhow error handling](https://google.github.io/comprehensive-rust/error-handling/anyhow.html)
##### R24 | [Iroh compatible error handling](https://www.iroh.computer/blog/iroh-0-95-0-new-relay)
##### R25 | [tracing-subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/)


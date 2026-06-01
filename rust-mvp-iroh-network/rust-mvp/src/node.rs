/* 
`node.rs` exposes four public functions and a constant.
References: 
    Creating an Endpoint: https://docs.iroh.computer/connecting/creating-endpoint 
    iroh example listen.rs: https://github.com/n0-computer/iroh/blob/main/iroh/examples/listen.rs
    iroh example connect.rs: https://github.com/n0-computer/iroh/blob/main/iroh/examples/connect.rs
    iroh sendme example: https://github.com/n0-computer/sendme
    iroh protocols: https://docs.iroh.computer/concepts/protocols
    iroh blobs: https://docs.iroh.computer/protocols/blobs
    iroh tickets: https://docs.iroh.computer/concepts/tickets
    iroh example transfer.rs: https://github.com/n0-computer/iroh-blobs/blob/main/examples/transfer.rs
*/
use iroh::{Endpoint, EndpointAddr, endpoint::presets, protocol::Router};
use iroh_blobs::{store::mem::MemStore, ticket::BlobTicket, BlobsProtocol};
use n0_error::{Result, StdResultExt};
use std::path::PathBuf;

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
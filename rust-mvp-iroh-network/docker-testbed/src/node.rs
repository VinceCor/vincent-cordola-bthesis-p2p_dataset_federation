// Code taken from rust-mvp (peer function)
use iroh::{Endpoint, EndpointAddr, endpoint::presets, protocol::Router};
use iroh_blobs::{BlobsProtocol, store::mem::MemStore, ticket::BlobTicket};
use n0_error::{Result, StdResultExt};
use std::path::PathBuf;

// peer: single process that simultaneously serves local files and accepts interactive fetch commands.
// 
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

    // The `while let Some` section whas created with the help of Claude chatbot
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
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
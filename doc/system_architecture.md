# System Architecture
This document provides an overview of the components used in this project. It describes what each component does and how they communicate with one another. Every institution runs an identical software stack.

This document will be updated throughout the project.

## Table of Contents
1. [Global topology](#1-global-topology)

## 1. Global Topology
Here is an overview of the project's overall architecture.  
![system architecture made with draw.io](media/architecture_p2p(1).png)  
*system architecture made with draw.io [R1](#r1--drawio)*



---

### 1.1  Transport layer
> References: [iroh-blobs](https://docs.iroh.computer/protocols/blobs), [Async Rust Challenges in Iroh](https://www.iroh.computer/blog/async-rust-challenges-in-iroh), [iroh NAT Traversal - concepts](https://docs.iroh.computer/concepts/nat-traversal), [iroh on QUIC Multipath](https://www.iroh.computer/blog/iroh-on-QUIC-multipath), [iroh - Using QUIC](https://docs.iroh.computer/protocols/using-quic)
#### 1.11 Role       
The  Transport layer is the only component that communicates with iroh-blobs protocol. It is responsible for:
- Joining the ad-hoc iroh network and maintaining connectivity to other peers.
- Send local Parquet files to other peer upon request
- Retrieve Parquet files from other peers
- Expose a local HTTP API so that the Python layer can trigger thses operations without needing to know anything about the network.

#### 1.12 Components
| Component | Role | References |
|---|---|---|
| Rust + Tokio | Asynchronous runtime | [tokio.rs](https://tokio.rs/) |
| iroh | P2P Network toolkit | [iroh](https://www.iroh.computer/) |
| iroh-blobs | Blob transfer | [iroh-blobs](https://docs.iroh.computer/protocols/blobs) |
| PyO3 (not implemented) | Bindings Python/Rust | [PyO3](https://pyo3.rs/v0.28.3/) |
| iroh-gossip (not implemented) | Manifest propagation between nodes | [iroh-gossip](https://docs.iroh.computer/connecting/gossip) | 
| Axum (not implemented) | A lightweight HTTP framework for Rust, build on Tokio. It exposes a local API on `localhost:x` so that the Python layer can interact with the Rust Node without needing to know anything about iroh | [axum](https://github.com/tokio-rs/axum) |

#### 1.13 internal architecture
The Node Agent is built around four iroh components that are instantiated at startup and shared for the entire duration of the process.

`Endpoint` the single entry point for the iroh network. It manages the underlying UDP socket and all network logic: relay, hole punching, and QUIC. All other components are linked to it.

`Router` replaces the manual acceptance loop. It runs in the background in Tokio and routes each incoming connection to the correct protocol handler based on the ALPN identifier negotiated during the QUIC handshake. In this project, the only registered protocol is `iroh_blobs::ALPN`.

`MemStore` in-memory storage of BLAKE3 blobs and their metadata. It servers as a receive buffer during downloads (blobs are verified there before being exported to disk) and as a source during uploads. A tag kept alive in the store prevents the iroh-blobs garbage collector from deleting the blob.

`Downloader` coordinates downloads from one or more peers. By being reused for multiple files, it allows iroh to maintain the existing QUIC connection to a given peer, thereby avoiding the overhead of a new handshake for each file.

#### 1.14 HTTP API exposed to Python

| Endpoint | Method | Description |
|---|---|---|
| `/health` | GET | Returns the node's status |
| `/files` | GET | Returns the merged manifest: local files + files known from peers |
| `/fetch` | POST | Triggers an iroh-blobs download using the specified ticket. Returns the cached local path once the download is complete |

#### 1.15 JSON manifest
This document will indicate what data the peer possesses. It will also allow you to retrieve the data by associating it with the corresponding ticket number.

```JSON
{
    "file_name": "bold_1.parquet",
    "ticket": "docaaacarwhmusoqf362j3jpzrehzkw3bqamcp2mmbhn3fmag3mzzfjp4beahj2v7aezhojvfqi5wltr4v"
}
```

#### 1.16 Data flow
Compared to the [iroh setup guide](../rust-mvp-iroh-network/iroh_setup_guide.md), different methods have been developed for using iroh. The `peer` version is the final version.

**listen**  
1. When it starts up, the Node Agent scans the `data/` folder and filters for `.paquet` files.
2. For each file, `store.blobs().add_path(abs_path)` reads the file, calculates its BLAKE3 hash, and stores the metadata in `MemStore`. The return value is a `Tag` (hash + format)
3. A `BlobTicket` is generated using the `tag` and the local `endpoint` address. This ticket is the only information a peer needs to retrieve the file: it conains the node address and the BLAK3 hash of the blob.
4. The `Router` is started (`.spawn()`): it runs in the background and handles incoming requests.

**fetch**   
1. The peer receives one or more `BlobTickets` (passed as CLI argumnets, or eventually propagated via iroh-gossip).
2. The `Downloader` establishes an iroh connection to the node specified in the ticket and downloads the blob to the local `MemStore`.
3. `store.blobs().export(hash, dest)` copies the verified blob from memory to disk, into the cache/ folder.

**peer**    
In `peer` mode, both streams coexist within the same Tokio process. The `Router` runs as a background task while the interactive loop waits for user commands in a non-blocking manner. The `Endpoint` and `MemStore` are shared between the two roles.

#### 1.17 NAT traversal


---

### 1.2 Cache layer
The Cache layer acts as a bridge between the Transport layer and the Request layer. It is responsible for:
- Determining which files are already present locally (so they don't need to be fetched again)
- Trigerring a fetech via the Transport layer if a requested file is missing
- Maintaining manifest.json
- Presenting the cache as a simple local directory to the Request layer.

#### 1.21 Components
| Component | Role |
|---|---|
| `cache/` directory | Directory on disk. Contains one .parquet file per downloaded file, named after its BLAKE 3 hash |
| `manifest.json` | Metadata registry. Currently associates each hash with its filename | []() |
| P2PClient | A lightweight Python wrapper around the `Axum` HTTP API. It can call `/health`, `/files` and `/fetch` |
| P2PDataset | Manages the cache. Calls `P2PClient.fetch()` if a file is missing, updates `manifest.json` after each fetch, and returns the local paths to the query layer |

#### 1.22 How it works
```
Is the file in cache/
    - YES -> Rturn the local path directly
    - NO -> Call P2PClient.fetch(ticket)
            -> Rust node download the file via iroh
                -> Update manifest.json
                    -> Return the local path
```

### 1.3 Query layer
The query layer is the one with which the researcher interacts. It is responsible for:
- Providing a simple Python API (`p2p.load()`,`p2p.files()`) that hides all complexity
- Execute queries on multiple Parquet files from multiple peers
- Return standard (pandas) DataFrames

#### 1.31 Components
| Component | Role | References |
|---|---|---|
| DuckDB? | | []() |
| Polars? | | []() |
| pandas | Standard Python data analysis library | [pandas](https://pandas.pydata.org/docs/) |
| `p2p.load(filename)` | Request the cache layer to get the local path -> reads it -> returns a Dataframe | |
| `p2p.files()` | Returns all available files | |

## References

| # | Source |
|---|--------|
| R1 | [draw.io](https://www.drawio.com/) |
| R2 | [iroh-blobs](https://docs.iroh.computer/protocols/blobs) |
| R3 | [Async Rust Challenges in Iroh](https://www.iroh.computer/blog/async-rust-challenges-in-iroh) |
| R4 | [iroh NAT Traversal - concepts](https://docs.iroh.computer/concepts/nat-traversal) |
| R5 | [iroh on QUIC Multipath](https://www.iroh.computer/blog/iroh-on-QUIC-multipath) |
| R6 | [iroh - Using QUIC](https://docs.iroh.computer/protocols/using-quic) |

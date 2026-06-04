# System Architecture
This document provides an overview of the components used in this project. It describes what each component does and how they communicate with one another. Every institution runs an identical software stack.

This document will be updated throughout the project.

## Table of Contents
1. [Global topology](#1-global-topology)

## 1. Global Topology
Here is an overview of the project's overall architecture.
![system architecture made with draw.io](media/system_architecture.drawio.png)  
*system architecture made with draw.io [R1](#r1--drawio)*

---

### 1.1 Node agent
> References: [iroh-blobs](https://docs.iroh.computer/protocols/blobs), [Async Rust Challenges in Iroh](https://www.iroh.computer/blog/async-rust-challenges-in-iroh), [iroh NAT Traversal - concepts](https://docs.iroh.computer/concepts/nat-traversal), [iroh on QUIC Multipath](https://www.iroh.computer/blog/iroh-on-QUIC-multipath), [iroh - Using QUIC](https://docs.iroh.computer/protocols/using-quic)
#### 1.11 Role       
The Node Agent is the only component that communicates with iroh-blobs protocol. It is responsible for:
- Joining the ad-hoc iroh network and maintaining connectivity to other peers.
- Send local Parquet files to other peer upon request
- Retrieve Parquet files from other peers

#### 1.12 Technology
| Tool | Role | References |
|---|---|---|
| Rust + Tokio | Asynchronous runtime | [tokio.rs](https://tokio.rs/) |
| iroh | P2P Network toolkit | [iroh](https://www.iroh.computer/) |
| iroh-blobs | Blob transfer | [iroh-blobs](https://docs.iroh.computer/protocols/blobs) |
| PyO3 (not implemented) | Bindings Python/Rust | [PyO3](https://pyo3.rs/v0.28.3/) |
| iroh-gossip (not implemented) | Manifest propagation between nodes | [iroh-gossip](https://docs.iroh.computer/connecting/gossip) | 

#### 1.13 internal architecture
The Node Agent is built around four iroh components that are instantiated at startup and shared for the entire duration of the process.

`Endpoint` the single entry point for the iroh network. It manages the underlying UDP socket and all network logic: relay, hole punching, and QUIC. All other components are linked to it.

`Router` replaces the manual acceptance loop. It runs in the background in Tokio and routes each incoming connection to the correct protocol handler based on the ALPN identifier negotiated during the QUIC handshake. In this project, the only registered protocol is `iroh_blobs::ALPN`.

`MemStore` in-memory storage of BLAKE3 blobs and their metadata. It servers as a receive buffer during downloads (blobs are verified there before being exported to disk) and as a source during uploads. A tag kept alive in the store prevents the iroh-blobs garbage collector from deleting the blob.

`Downloader` coordinates downloads from one or more peers. By being reused for multiple files, it allows iroh to maintain the existing QUIC connection to a given peer, thereby avoiding the overhead of a new handshake for each file.

#### 1.14 Data flow
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

#### 1.15 NAT traversal


---

### 1.2 Parquet Files
#### Role
Each institution stores its data as parquet files in a local `data/` directory.

---

### 1.3 Local cache
#### Role
The local cache is a directory on disk (`cache/`) where files downloaded from remote peers are stored. Its purpose is to make the network transparent to the Python layer.

---

### 1.4 Python client
#### Role
The Python client ats as a bridge between the node agent and the Jupyter notebook.


---

### 1.5 Jupyter Notebook
#### Role
The Jupyter notebook is the main demonstration artifact of this PoC. It represents the researcher's view: a clean interface, with no awareness of the network, that loads data from what appears to be local dataset.


## References

| # | Source |
|---|--------|
| R1 | [draw.io](https://www.drawio.com/) |
| R2 | [iroh-blobs](https://docs.iroh.computer/protocols/blobs) |
| R3 | [Async Rust Challenges in Iroh](https://www.iroh.computer/blog/async-rust-challenges-in-iroh) |
| R4 | [iroh NAT Traversal - concepts](https://docs.iroh.computer/concepts/nat-traversal) |
| R5 | [iroh on QUIC Multipath](https://www.iroh.computer/blog/iroh-on-QUIC-multipath) |
| R6 | [iroh - Using QUIC](https://docs.iroh.computer/protocols/using-quic) |

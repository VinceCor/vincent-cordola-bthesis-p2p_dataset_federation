# System Architecture
This document provides an overview of the components used in this project. It describes what each component does and how they communicate with one another. Every institution runs an identical software stack.

This document will be updated throughout the project.

## Table of Contents

## 1. Global Topology
Here is an overview of the project's overall architecture.
![system architecture made with draw.io](media/system_architecture.drawio.png)  
*system architecture made with draw.io [R1](#r1--drawio)*

### 1.1 Node agent
#### Role       
The Node Agent is the only component that communicates with iroh-blobs protocol. It is responsible for:
- Joining the ad-hoc iroh network and maintaining connectivity to other peers.
- Publish the `manifest.json` file to the network for other nodes
- Send local Parquet files to other peer upon request
- Retrieve Parquet files from other peers

#### Technology
| Tool | Role | References |
|---|---|---|
| Rust + Tokio | Asynchronous runtime | [tokio.rs](https://tokio.rs/) |
| iroh | P2P Network toolkit | [iroh](https://www.iroh.computer/) |
| iroh-blobs | Blob transfer | [iroh-blobs](https://docs.iroh.computer/protocols/blobs) |
| PyO3 | Bindings Python/Rust | [PyO3](https://pyo3.rs/v0.28.3/) |
| iroh-gossip | Manifest propagation between nodes | [iroh-gossip](https://docs.iroh.computer/connecting/gossip) | 


### 1.2 Parquet Files
#### Role
Each institution stores its data as parquet files in a local `data/` directory.

### 1.3 Local cache
#### Role
The local cache is a directory on disk (`cache/`) where files downloaded from remote peers are stored. Its purpose is to make the network transparent to the Python layer.

### 1.4 Python client
#### Role
The Python client ats as a bridge between the node agent and the Jupyter notebook.

### 1.5 Jupyter Notebook
#### Role



## References
##### R1 | [draw.io](https://www.drawio.com/)
##### R2 | [Iroh blobs](https://docs.iroh.computer/protocols/blobs)
##### R3 | [Async Rust Challenges in Iroh](https://www.iroh.computer/blog/async-rust-challenges-in-iroh)

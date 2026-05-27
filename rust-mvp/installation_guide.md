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


### 2.2 main.rs
> References: [Creating an Endpoint](https://docs.iroh.computer/connecting/creating-endpoint), [iroh example listen.rs](https://github.com/n0-computer/iroh/blob/main/iroh/examples/listen.rs) and [iroh example connect.rs](https://github.com/n0-computer/iroh/blob/main/iroh/examples/connect.rs)

### 2.3 node.rs




## 3. todo
> iroh-blobs: https://docs.iroh.computer/protocols/blobs    
iroh-blobs example: transfer.rs https://github.com/n0-computer/iroh-blobs/blob/main/examples/transfer.rs    
sendme (iroh blobs file transfer example) https://github.com/n0-computer/sendme





## References
##### R1 | [Rust and Cargo installation](https://doc.rust-lang.org/cargo/getting-started/installation.html)
##### R2 | [What is iroh?](https://docs.iroh.computer/what-is-iroh)
##### R3 | [iroh quickstart](https://docs.iroh.computer/quickstart)
##### R4 | [Creating an Endpoint](https://docs.iroh.computer/connecting/creating-endpoint)
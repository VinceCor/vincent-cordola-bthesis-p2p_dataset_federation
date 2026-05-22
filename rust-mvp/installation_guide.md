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

### 2.1 Add dependencies
Add dependencies (this can be done directory in Cargo.toml if you want a precise version)
> References: [iroh crates](https://crates.io/crates/iroh) and [tokio.rs](https://tokio.rs/)
```bash
cargo add iroh # Iroh
cargo add tokio --features full # asynchronous Rust runtime, required for iroh
```

### 2.2 Create an endpoint
> References: [Creating an Endpoint](https://docs.iroh.computer/connecting/creating-endpoint), [iroh example listen.rs](https://github.com/n0-computer/iroh/blob/main/iroh/examples/listen.rs) and [iroh example connect.rs](https://github.com/n0-computer/iroh/blob/main/iroh/examples/connect.rs)


## 3. todo
> iroh-blobs: https://docs.iroh.computer/protocols/blobs    
iroh-blobs example: transfer.rs https://github.com/n0-computer/iroh-blobs/blob/main/examples/transfer.rs    





## References
##### R1 | [Rust and Cargo installation](https://doc.rust-lang.org/cargo/getting-started/installation.html)
##### R2 | [What is iroh?](https://docs.iroh.computer/what-is-iroh)
##### R3 | [iroh quickstart](https://docs.iroh.computer/quickstart)
# Installation and usage guide
## 1. Overview
This guide help setting up the project from a fresh Linux machine (or VM) to running a frist end-to-end demo in a jupyter notebook: build the Rust node, start it as a peer, set up the Python client, and query the federated dataset.

The deployment model is one peer per machine: each institution runs a single peer process on its own machine/VM. This guide follows that model throughout.

## 2. Prerequisites
| Component | Why | Reference |
|---|---|---|
| Linux distribution | Target OS for this guide (any recent distro, tested assumptions: Debian/Ubuntu like) | - |
| Rust | Builds and runs the node | [rustup](https://rust-lang.org/tools/install/) |
| Python 3.10+ | Runs the client layer and jupyter | [python.org download](https://www.python.org/downloads/) |
| `pip`/ `venv` | Python dependency | included with Python 3 |
| `git` | Cloning the repository | your distro's package manager |
Nothin else needs to be installed manually, the Tust dependencies (iroh, iroh-blobs, iroh-gossip, Axum, ...) are fetched automatically by `cargo build`, and the Python dependencies by `pip install -r requirements.txt`

## 3. Installing Rust
Follow the official instructions https://rust-lang.org/tools/install/. Once installed, open a new terminal and check:
```bash
rustc --version
cargo --version
```
Both should print a version number

## 4. Getting the project
```bash
git clone https://github.com/VinceCor/vincent-cordola-bthesis-p2p_dataset_federation
cd vincent-cordola-bthesis-p2p_dataset_federation
```
The project tree is described in [README.md](../README.md). The two parts you will work wite are `peer-dataset-federation/node/` and `peer-dataset-federation/client/`. The `doc/` folder holds the design documents, `rust-mvp-iroh-network` allows you to see how the Rust implementation (iroh) was carried out, not needed to run the final notebook.

## 5. Building the Rust node
```bash
cd peer-dataset-federation/node
cargo build
```
The first build will take a while: `cargo` compiles iroh and its dependencies from scratch. Subsequent builds are much faster.

## 6. Running your first peer
A "peer" is a single process that both serves your local Parquet files to the network and lets you fetch files from other peers. Configuration is passed via environment variables.
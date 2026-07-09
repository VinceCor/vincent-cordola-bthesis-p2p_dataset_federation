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

| Variable | Required | Meaning |
|---|---|---|
| `INSTITUTION` | yes | Name advertised in this node's manifest (e.g. `mcgill`, `hes-so`) |
| `BOOTSTRAP_PEERS` | no | Comma separated `EndpointId` to join an existing network. Leave unset if you are the first peer |

Place any `.parquet` file you want to share in `peer-dataset-federation/node/data/`.  
### 6.1 first peer to start
Then in your terminal
```bash
INSTITUTION=<INSTITUTION> cargo run -- peer
```
You should see something like:
```bash
Peer: EndpointAddr { id: PublicKey(c097e791bb83c730da69cc8c421c17b5e30909659037276ab5fe85cb6071eab5), addrs: {Ip(172.18.180.164:53468)} }
Institution: peer1
File hashing in data/
 [sample.parquet] ticket: blobadajpz4rxob4omg2nhgiyqq4c626gcijmwidoj3kwx7ils3aohvlkaaaads4zm2lu4tk7vmffi7iw2tmobht43yhjstgv4rr7xc4r3e76nkf6
Joining gossip topic with 0 bootstrap peer(s)
Gossip: manifest broadcast for institution 'peer1'
Router started. Type 'fetch <ticket>' or 'quit'
HTTP API listening on http://0.0.0.0:8080
```
The `PublicKey(...)` value in the first line is this peer's `EndpointId`, another peer will need it to join your network via `BOOTSTRAP_PEERS`.

### 6.2 Joining an existing network
if someone else already started a peer dans gave you their `EndpointId`.
```bash
INSTITUTION=<INSTITUTION> BOOTSTRAP_PEERS=<their_endpoint_id> cargo run -- peer
```

Leave this terminal running: the Router, the gossip listener, and the HTTP API all live in this process.

## 7. Verifying the node (HTTP API)
With the peer still running, open a second terminal:
```bash
curl http://localhost:8080/health
# {"institution":"peer1","status":"ok"}

curl http://localhost:8080/files
# {"manifests":[{"files":[{"file_name":"sample.parquet","hash":"e5ccb34ba72...","ticket":"blobacjg2uak3..."}],"institution":"peer1"}]}
```
`/fetch` is normally called by the Python client, but you can test it directly by copying a `ticket` value from the `/files` response above:
```bash
curl -X POST http://localhost:8080/fetch \
     -H "Content-Type: application/json" \
     -d '{"ticket": "<ticket_from_files>"}'
# {"path":"cache/3b9de451872ec4a1.parquet"}
```

## 8. Setting up the Python client
```bash
cd peer-dataset-federation/client
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```
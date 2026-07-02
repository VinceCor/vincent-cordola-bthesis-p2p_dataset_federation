# Python HTTP client

## 1. Overview
> References: [Requests](https://requests.readthedocs.io/en/latest/)

The Rust node exposes three HTTP endpoints (`/health`, `/files`, `/fetch`), documented in the Axum HTTP server guide. `P2PClient` is the only piece of Python code that knows this node exists. It does not interpret the data it receives, it does not decide whteher a file should be downloade, in only translates Python method calls into HTTP requests and HTTP responses into Python values.

## 2. Dependencies
`requests` the standard synchronous HTTP library for Python. Chosen over `AIOHTTP` or `HTTPX` because the project doesn't need async HTTP calls on the Python side.
```bash
pip install requests
```

## 3. client.py
### 3.1 P2PError
```Python
class P2PError(Exception):
    # Raised when the Rust node returns an error or is unreachable
    pass
```
The caller (`P2PDataset`, or the researcher directly in a notebook) only needs to know "the node call failed", not whether it was a connection error, a timeout, or an HTTP 500. Catching one exception type is simple than catching three.

### 3.2 The constructor
```Python
def __init__(self, base_url: str = "http://localhost:8080", timeout: int = 30):
    # base_url -> base url of the Rust node
    self.base_url = base_url.rstrip("/")
    # timeout -> Request timeout in seconds
    self.timeout = timeout
```
`base_url.rstrip("/")` removes a trailing slash if the user passes `http://localhost:8080/`. This avoids accidentally producing `http://localhost:8080//` when concatenating with `"/health"` later.    
`timeout` defaults to 30 seconds. `GET /health` and `GET /files` are fast (local disk reads), but `POST /fetch` triggers an actual iroh-blobs download, which depends on file size and network conditions (direct connection vs relay fallback). A single shared timeout keeps the class simple for the PoC, if `/fetch` later needs a longer timeout than `/health`, this can be split.

### 3.3 health(), files(), fetch()
These three public methods are the entire surface of the class. Each one calls a private helper (`get` or `post`) and shapes the return value
```Python
# GET /health
# Check that the Rust node is running
def health(self) -> dict:
    return self.get("/health")

# GET /files
# Return the merged manifest: local files + all peers manifests
def files(self) -> dict:
    return self.get("/files")

# POST /fetch
# Ask the Rust node to download the blob identified by <ticket>
def fetch(self, ticket: str) -> str:
    data = self.post("/fetch", {"ticket": ticket})
    return data["path"]
```
`health()` and `files()` return the JSON body.  
`fetch()` is the only method that unwraps the response. The Rust node returns `{"path": "cache/<hash>.parquet"}`, returning `data["path"]` directly instead of the whole dict.

### 3.4 Private helpers
> References: [requests errors and exceptions](https://docs.python-requests.org/en/latest/user/quickstart/#errors-and-exceptions), [requests timeout](https://docs.python-requests.org/en/latest/user/advanced/#timeouts)

This is the part of the file doing the actual work, `get` and `post` are nearly identical, which is intentional.
```Python
def get(self, path: str) -> dict:
    url = self.base_url + path

    try:
        response = requests.get(url, timeout=self.timeout)
    except requests.exceptions.ConnectionError as e:
        raise P2PError(f"Cannot reach the Rust node at {self.base_url}") from e
    except requests.exceptions.Timeout:
        raise P2PError(f"Request timed out after {self.timeout}s: GET {url}")

    self.raise_for_status(response)
    return response.json()
```
```Python
def post(self, path: str, body: dict) -> dict:
    url = self.base_url + path

    try:
        response = requests.post(url, json=body, timeout=self.timeout)
    except requests.exceptions.ConnectionError as e:
        raise P2PError(f"Cannot reach the Rust node at {self.base_url}") from e
    except requests.exceptions.Timeout:
        raise P2PError(f"Request timed out after {self.timeout}s: GET {url}")
    
    self.raise_for_status(response)
    return response.json()
```

For two call sites with identical error handling, the duplication here is small enough that splitting by verb keeps each method readable without an extra layer of indirection. If a third HTTP verb were needed, merging into one `request()` would become the better trade-off.   
**Two layers of error handling** The `try/except` block catches transport-level failures.
- `ConnectionError`: the node isn't listening on that port (not started, wrong port, firewall)
- `Timeout`: the node is reachable but didn't answer within `self.timeout` seconds

`self.raise_for_status(response)`, called after the `try/except`, catches application-level failures, the node did repond, but with a non 200 status (ex. the Axum `/fetch` handler returning `500` because the ticket was invalid)

### 3.5 raise for status
```Python
@staticmethod
def raise_for_status(response: requests.Response) -> None:
    if response.status_code != 200:
        try:
            detail = response.json().get("error", response.text)
        except Exception:
            detail = response.text
        raise P2PError(f"Rust node returned HTTP {response.status_code}: {detail}")
```
It inspects a `Response` object and converts a non 200 status into a `P2PError` with a useful message. It tries to extract the "error" filed the Rust API puts in its JSON error bodies, and falls back to the raw response text if the body isn't JSON, or doesn't contain that field.





### 3.6 Result
#### 3.61 Install python library
If you haven't alreadym install the requests library.
```bash
# Example of how to reate a .venv
python3 -m venv .venv

source .venv/bin/activate

pip install requests
# leave
deactivate
```

#### 3.62 Manual test
With the Rust node running (`INSTITUTION=peer1 cargo run --peer`)   
`python3 test_client.py`
```Python
# Test client
from p2p.client import P2PClient

client = P2PClient("http://localhost:8080")
print(client.health())

print(client.files())
```

#### 3.63 Testing the error path
With the node stopped:
```Python
try:
    client.health()
except P2PError as e:
    print(e)
```

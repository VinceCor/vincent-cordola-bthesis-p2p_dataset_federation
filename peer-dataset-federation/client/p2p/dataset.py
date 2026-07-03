# P2PDataset
# Fetch on demand access to distributed Parquet dataset

import logging
from pathlib import Path

from .client import P2PClient, P2PError

logger = logging.getLogger(__name__)

cache_dir = (Path(__file__).resolve().parent / "../../node/cache").resolve()

# Fetch on demand access to the distributed Parquet dataset
class P2PDataset:

    def __init__(self, client: P2PClient):
        self._client = client

    # List every file visible across all peer manifests
    def files(self) -> list[dict]:
        response = self._client.files()
        return [
            {"file_name": f["file_name"], "institution": manifest.get("institution", "unknown")}
            for manifest in response.get("manifests", [])
            for f in manifest.get("files", [])
        ]        

    # Return a local Path to a *file_name*, fetching it from the network if it is not already cached.
    def get(self, file_name: str) -> Path | None:
        # Walk all peer manifests to find the file
        response = self._client.files()
        entry = None
        for manifest in response.get("manifests", []):
            for f in manifest.get("files", []):
                if f["file_name"] == file_name:
                    entry = f | {"institution": manifest.get("institution", "unknown")}
                    break
            if entry is not None:
                break
            
        if entry is None:
            logger.warning("'%s' not found in any peer manifest", file_name)
            return None
        
        # Derive the expected cache path from the BLAKE3 hash
        cached = cache_dir / f"{entry['hash'][:16]}.parquet"

        if cached.exists():
            logger.info("cache hit: %s", file_name)
            return cached
        
        # Not on disk: fetch via the Rust node
        logger.info("fetching '%s' from %s ...", file_name, entry["institution"])
        self._client.fetch(entry["ticket"])
        logger.info("saved -> %s", cached)

        return cached

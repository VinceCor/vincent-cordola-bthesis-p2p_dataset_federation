# Test client
import logging
import pandas as pd
from p2p.client import P2PClient
from p2p.dataset import P2PDataset

logging.basicConfig(level=logging.INFO)

client = P2PClient("http://localhost:8080")
dataset = P2PDataset(client)

print(dataset.files())

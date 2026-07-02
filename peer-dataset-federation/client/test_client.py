# Test client
from p2p.client import P2PClient

client = P2PClient("http://localhost:8080")
print(client.health())

print(client.files())

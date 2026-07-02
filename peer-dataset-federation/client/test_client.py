# Test client
from p2p.client import P2PClient, P2PError

client = P2PClient("http://localhost:8080")
print(client.health())

print(client.files())

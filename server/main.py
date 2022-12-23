import socket
from lib.manage import Connection

s = socket.socket()
s.bind(("0.0.0.0", 1234))
s.listen(1)

print(f"[i] Listening on 0.0.0.0:1234")

c, a = s.accept()
Connection(c).listen()

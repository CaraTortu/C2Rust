import socket, threading
from lib.manage import Connection

s = socket.socket()
s.bind(("0.0.0.0", 1234))
s.listen(1)

print(f"[i] Listening on 0.0.0.0:1234")

clients = []
threads = []

while True:
    c, a = s.accept()

    conn = Connection(c)
    clients.append(conn)

    t = threading.Thread(target=conn.listen)

    threads.append(t)
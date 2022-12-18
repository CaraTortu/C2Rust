import socket, threading
from manage import Connection

s = socket.socket()
s.bind(("127.0.0.1", 1234))
s.listen(1)

clients = []
threads = []

while True:
    c, a = s.accept()

    conn = Connection(c)
    clients.append(conn)

    t = threading.Thread(target=conn.listen)

    threads.append(t)
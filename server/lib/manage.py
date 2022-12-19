import socket

import lib.utils as utils
import lib.crypt as crypt

class Connection:
    def __init__(self, conn: socket.socket) -> None:
        self.conn = conn
        self.key = conn.recv(32)
        self.listen()

    def send_command(self, cmd: str) -> None:
        self.conn.send(cmd)

    def listen(self) -> None:
        print("Key:", self.key.decode())

        while True:
            x = input("$ ")

            nonce = utils.getRandomString(16).encode()
            enc = crypt.encrypt(x, nonce, self.key)
            self.send_command(enc + b"|" + nonce + b"\n")

            if x == "exit:":
                print("[i] Closed connection")
                break

            msg, nonce = self.conn.recv(1024).decode().split("|")

            print(crypt.decrypt(msg, nonce.encode(), self.key))

        exit(0)
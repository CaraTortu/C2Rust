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

            message = ""

            while True:
                message += self.conn.recv(512).decode()

                if "|" in message and len(message.split("|")[1]) >= 16:
                    break

            msg, nonce = message.split("|")

            a = crypt.decrypt(msg, nonce.encode(), self.key).replace("2937846nd726345dnh", "\n")
            print(a)

        exit(0)
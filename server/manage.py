import socket, base64, utils

from cryptography.hazmat.primitives.ciphers import (
    Cipher, algorithms, modes
)

class Connection:
    def __init__(self, conn: socket.socket) -> None:
        self.conn = conn
        self.key = conn.recv(32)
        self.listen()

    def send_command(self, cmd: str) -> None:
        self.conn.send(cmd)

    def encrypt(self, msg: str, nonce: bytes) -> bytes:
        encryptor = Cipher(algorithms.AES256(self.key), modes.CBC(nonce)).encryptor()
        ciphertext = encryptor.update(msg.encode()) + encryptor.finalize()

        return base64.b64encode(ciphertext)

    def decrypt(self, msg: bytes, nonce: bytes) -> str:
        msg = base64.b64decode(msg)
        decryptor = Cipher(algorithms.AES256(self.key), modes.CBC(nonce)).decryptor()
        return decryptor.update(msg)

    def listen(self) -> None:
        print("Key:", self.key.decode())

        while True:
            x = input("$ ")
            nonce = utils.getRandomString(16).encode()
            print("Nonce: " + nonce.decode())
            enc = self.encrypt(x, nonce)

            print("enc:", enc)

            self.send_command(enc + b"|" + nonce + b"\n")

            msg = self.conn.recv(1024)

            print("Message: " + msg.decode())
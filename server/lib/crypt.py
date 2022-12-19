
from Crypto.Cipher import AES
import base64

import lib.utils as utils

def pad(s: str):
    l = len(s)

    remainder = l // 16

    if remainder == 0: remainder = 32 - l
    else: remainder = ((remainder*2)*32) - l

    return s.encode() + (b'\x08' * remainder)

def encrypt(msg: str, iv: bytes, key: bytes):
    padded_text = pad(msg)
    cipher_config = AES.new(key, AES.MODE_CBC, iv)    
    ciper = cipher_config.encrypt(padded_text)
    return base64.b64encode(ciper)

def decrypt(msg: str, iv: bytes, key: bytes) -> str:
    msg = base64.b64decode(msg)
    cipher = AES.new(key, AES.MODE_CBC, iv)
    return utils.filter(cipher.decrypt(msg).decode())
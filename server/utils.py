import random

def getRandomString(n: int) -> str:
    return ''.join(random.choices(list("abcdefghijklmnoprstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"), k=n))

import random, itertools

def getRandomString(n: int) -> str:
    return ''.join(random.choices(list("abcdefghijklmnoprstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"), k=n))

def filter(s: str) -> str:
    nonprintable = itertools.chain(range(0x00,0x20),range(0x7f,0xa0))
    return s.translate({character:None for character in nonprintable})

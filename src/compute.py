import hashlib
from Crypto.Hash import keccak

def subslice_to_u64(s, begin, end):
    result = 0
    for i in range(begin, end):
        result <<= 8;
        result += s[i]
    return result

# s = (b"X" * 32 * 14)
s = b"X"
# h = hashlib.keccak256(s)
k = keccak.new(digest_bits=256)
k.update(s)
# h = hashlib.sha256(s)
d = k.digest()
# d = h.digest()

print(d.hex())
print([int(v) for v in d])

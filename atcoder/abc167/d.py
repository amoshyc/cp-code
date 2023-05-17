import numpy as np

N, K = map(int, input().split())
A = list(map(int, input().split()))
A = np.int32(A) - 1

nxt = np.zeros((70, N), dtype=int)
nxt[0] = A
for i in range(0, 69):
    nxt[i + 1] = nxt[i, nxt[i]]

pos = 0
for i, v in enumerate(reversed(bin(K)[2:])):
    if v == '1':
        pos = nxt[i, pos]

print(pos + 1)
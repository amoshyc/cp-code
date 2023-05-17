import numpy as np

N, K = map(int, input().split())
M = 10 ** 9 + 7
G = [list(map(int, input().split())) for _ in range(N)]
G = np.array(G, dtype=object)

base = G
res = np.eye(N, dtype=object)
while K > 0:
    if K & 1:
        res = res @ base
        res = res % M
    base = base @ base
    base = base % M
    K >>= 1

print(res.sum() % M)

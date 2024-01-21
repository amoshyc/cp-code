import numpy as np

def matmul(A, B):
    N, M = A.shape[0], B.shape[1]
    C = np.zeros((N, M), dtype=np.int64)
    for i in range(N):
        for j in range(M):
            C[i, j] = ((A[i, :] * B[:, j]) % mod).sum() % mod
    return C

N, K = map(int, input().split())
mod = 10 ** 9 + 7
G = [list(map(int, input().split())) for _ in range(N)]
G = np.array(G, dtype=np.int64)

base = G
res = np.eye(N, dtype=np.int64)
while K > 0:
    if K & 1:
        res = matmul(res, base)
    base = matmul(base, base)
    K >>= 1

print(res.sum() % mod)

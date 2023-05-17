import sys
import itertools
import numpy as np

read = sys.stdin.buffer.read
readline = sys.stdin.buffer.readline
readlines = sys.stdin.buffer.readlines

N, M, X = map(int, readline().split())
data = np.int32(read().rstrip().split()).reshape(N, M + 1)
C = data[:, 0]
A = data[:, 1:]

ans = 10**8
for choses in itertools.product([0, 1], repeat=N):
    row_mask = np.int32(choses) > 0
    if np.all(A[row_mask].sum(axis=0) >= X):
        ans = min(ans, C[row_mask].sum().item())

print(-1 if ans == 10**8 else ans)
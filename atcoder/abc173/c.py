import numpy as np
from itertools import product

H, W, K = map(int, input().split())

A = np.zeros((H, W), dtype=int)
for r in range(H):
    for c, v in enumerate(input()):
        A[r, c] = 1 if v == '#' else 0

ans = 0
ttl = A.sum().item()

for choses_r in product([0, 1], repeat=H):
    for choses_c in product([0, 1], repeat=W):
        mask = np.zeros_like(A)
        mask[np.flatnonzero(choses_r), :] = 1
        mask[:, np.flatnonzero(choses_c)] = 1

        if ttl - A[mask > 0].sum() == K:
            ans += 1

print(ans)

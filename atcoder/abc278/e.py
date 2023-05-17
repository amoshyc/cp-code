import numpy as np

V = 301
H, W, N, h, w = map(int, input().split())
A = [[int(x) for x in input().split()] for _ in range(H)]
P = np.zeros((H, W, V), dtype=np.int32)
P[0][0][A[0][0]] = 1
for r in range(1, H):
    P[r][0] = P[r - 1][0]
    P[r][0][A[r][0]] += 1
for c in range(1, W):
    P[0][c] = P[0][c - 1]
    P[0][c][A[0][c]] += 1
for r in range(1, H):
    for c in range(1, W):
        P[r][c] = P[r - 1][c] + P[r][c - 1] - P[r - 1][c - 1]
        P[r][c][A[r][c]] += 1

for r in range(H - h + 1):
    for c in range(W - w + 1):
        r1, r2, c1, c2 = r, r + h - 1, c, c + w - 1
        box = P[r2][c2].copy()
        if r1 >= 1 and c1 >= 1:
            box += P[r1 - 1][c1 - 1]
        if r1 >= 1:
            box -= P[r1 - 1][c2]
        if c1 >= 1:
            box -= P[r2][c1 - 1]
        # print(r1, r2, c1, c2, P[r2][c2][:6], box[:6])
        ans = P[H - 1][W - 1] - box
        ans = (ans > 0).sum().item()
        print(ans, end=' ')
    print()
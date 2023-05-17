H, W = map(int, input().split())
A = [[int(x) for x in input().split()] for _ in range(H)]

ans = True
for i1 in range(H):
    for i2 in range(i1 + 1, H):
        for j1 in range(W):
            for j2 in range(j1 + 1, W):
                if not (A[i1][j1] + A[i2][j2] <= A[i1][j2] + A[i2][j1]):
                    ans = False

print('Yes' if ans else 'No')
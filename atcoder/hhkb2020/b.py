H, W = map(int, input().split())
S = [input() for _ in range(H)]

ans = 0
for r in range(H):
    for c in range(W):
        if S[r][c] == '.':
            if r + 1 < H and S[r + 1][c] == '.':
                ans += 1
            if c + 1 < W and S[r][c + 1] == '.':
                ans += 1
print(ans)
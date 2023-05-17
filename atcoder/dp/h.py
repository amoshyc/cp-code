H, W = map(int, input().split())
A = [input() for _ in range(H)]
M = 10 ** 9 + 7

dp = [[0 for _ in range(W)] for _ in range(H)]
dp[0][0] = 1
for r in range(H):
    for c in range(W):
        if r + 1 < H and A[r + 1][c] == '.':
            dp[r + 1][c] += dp[r][c]
            dp[r + 1][c] %= M
        if c + 1 < W and A[r][c + 1] == '.':
            dp[r][c + 1] += dp[r][c]
            dp[r][c + 1] %= M
print(dp[H - 1][W - 1])
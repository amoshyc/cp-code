N, W = map(int, input().split())
ws, vs = [], []
for _ in range(N):
    w, v = map(int, input().split())
    ws.append(w)
    vs.append(v)

# dp[i, j] = maximum value while total weight is j and only [0, i) items are considered
dp = [[0 for _ in range(W + 1)] for _ in range(N + 1)]
for i in range(N):
    for j in range(W):
        # don't use item i
        dp[i + 1][j] = max(dp[i + 1][j], dp[i][j])
        # use item i
        if j + ws[i] <= W:
            dp[i + 1][j + ws[i]] = max(dp[i + 1][j + ws[i]], dp[i][j] + vs[i])

print(max(dp[N]))

N, W = map(int, input().split())
ws, vs = [], []
for _ in range(N):
    w, v = map(int, input().split())
    ws.append(w)
    vs.append(v)
V = max(vs) * N

# dp[i, j] = minimum weight when total value is j and only items [0, i) is considered
INF = 10 ** 10
dp = [[INF for _ in range(V + 1)] for _ in range(N + 1)]
dp[0][0] = 0
for i in range(N):
    for j in range(V + 1):
        # don't use item i
        dp[i + 1][j] = min(dp[i + 1][j], dp[i][j])
        # use item i
        if j + vs[i] <= V:
            dp[i + 1][j + vs[i]] = min(dp[i + 1][j + vs[i]], dp[i][j] + ws[i])

for v in range(V, -1, -1):
    if dp[N][v] <= W:
        print(v)
        break

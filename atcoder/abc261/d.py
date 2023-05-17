N, M = map(int, input().split())
X = [int(x) for x in input().split()]
P = [0 for _ in range(N + 1)]
A = [0 for _ in range(N + 1)]
for _ in range(M):
    c, y = map(int, input().split())
    A[c] = y
P[0] = A[0]
for i in range(1, N + 1):
    P[i] = P[i - 1] + A[i]

pref = [X[0]]
for i in range(1, N):
    pref.append(pref[-1] + X[i])

dp = [[0, 0] for _ in range(N)]
dp[0][0] = 0
dp[0][1] = X[0] + A[1]
for i in range(1, N):
    dp[i][0] = max(dp[i - 1][0], dp[i - 1][1])
    for j in range(i):
        dp[i][1] = max(dp[i][1], dp[j][0] + P[i - j] + pref[i] - pref[j])
    dp[i][1] = max(dp[i][1], P[i + 1] + pref[i])
print(max(dp[-1]))
def cumsum(A):
    B = [A[0]]
    for i in range(1, len(A)):
        B.append(B[-1] + A[i])
    return B

N, M = map(int, input().split())
X = [int(x) for x in input().split()]
Y = [0 for _ in range(N + 1)]
for _ in range(M):
    c, y = map(int, input().split())
    Y[c] = y

Py = cumsum(Y)
Px = cumsum(X)

dp = [[0, 0] for _ in range(N)]
dp[0][0] = 0
dp[0][1] = X[0] + Y[1]
for i in range(1, N):
    dp[i][0] = max(dp[i - 1][0], dp[i - 1][1])
    dp[i][1] = max(
        Py[i + 1] + Px[i],
        max(dp[j][0] + Py[i - j] + Px[i] - Px[j] for j in range(i))
    )
print(max(dp[-1]))
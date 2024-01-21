N, X = map(int, input().split())
A = [int(x) for x in input().split()]

mx = []
for i in range(N - 1):
    mx.append(A[i + 1] // A[i])
mx.append(10 ** 18)

x = []
for a in reversed(A):
    q, X = divmod(X, a)
    x.append(q)
x = x[::-1]

dp = [[0, 0] for _ in range(N + 1)]
dp[0][0] = 1

for i in range(N):
    for c in range(2):
        # r[i] = 0 and y[i] = 0
        if (c + x[i]) % mx[i] == 0:
            new_c = (c + x[i]) // mx[i]
            dp[i + 1][new_c] += dp[i][c]

        # r[i] = 0 and y[i] ≠ 0
        if (c + x[i]) % mx[i] != 0:
            new_c = (c + x[i]) // mx[i]
            dp[i + 1][new_c] += dp[i][c]
        
        # r[i] ≠ 0 and y[i] = 0
        if (c + x[i]) % mx[i] != 0:
            dp[i + 1][1] += dp[i][c]

print(dp[N][0])
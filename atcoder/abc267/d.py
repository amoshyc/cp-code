import math

N, M = map(int, input().split())
A = [int(x) for x in input().split()]

# dp[i, j] = 使用前 i + 1 個元素，長度為 j 的 subsequences 中，所能組出的最大值
# 考慮第 i 項用或不用：
# dp[i, j] = max(dp[i - 1, j], dp[i - 1, j - A[i]] + j * A[i])
dp = [[-math.inf for _ in range(M + 1)] for _ in range(N)]

for i in range(N):
    dp[i][0] = 0

dp[0][1] = A[0]
for i in range(1, N):
    for j in range(1, M + 1):
        dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - 1] + j * A[i])
print(dp[N - 1][M])
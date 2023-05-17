N, K = map(int, input().split())
A = [int(x) for x in input().split()]
B = [int(x) for x in input().split()]

# dp[i, 0] = 前 i + 1 項能不能滿足，且第 i 項選 A
# dp[i, 1] = 前 i + 1 項能不能滿足，且第 i 項選 B

dp = [[False, False] for _ in range(N)]
dp[0] = [True, True]
for i in range(1, N):
    if abs(A[i] - A[i - 1]) <= K:
        dp[i][0] |= dp[i - 1][0]
    if abs(A[i] - B[i - 1]) <= K:
        dp[i][0] |= dp[i - 1][1]
    if abs(B[i] - A[i - 1]) <= K:
        dp[i][1] |= dp[i - 1][0]
    if abs(B[i] - B[i - 1]) <= K:
        dp[i][1] |= dp[i - 1][1]

print('Yes' if dp[-1][0] or dp[-1][1] else 'No')
N, M = map(int, input().split())
mod = 10 ** 9 + 7

dp = [0 for _ in range(N + 1)]
for _ in range(M):
    a = int(input())
    dp[a] = -1

dp[0] = 1 if dp[0] == 0 else 0
dp[1] = dp[0] if dp[1] == 0 else 0

for i in range(2, N + 1):
    if dp[i] == -1:
        dp[i] = 0
    else:
        dp[i] = (dp[i - 1] + dp[i - 2]) % mod

print(dp[-1])

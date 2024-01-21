N = int(input())
M = 10**9 + 7

if N < 3:
    print(0)
elif N < 6:
    print(1)
else:
    dp = [0] * (N + 1)
    dp[3] = 1
    dp[4] = 1
    dp[5] = 1
    P = dp[3]
    for i in range(6, N + 1):
        dp[i] = (P + 1) % M
        P = (P + dp[i - 2]) % M
    print(dp[-1] % M)
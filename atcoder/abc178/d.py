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
    for i in range(6, N + 1):
        dp[i] = sum(dp[3:i - 3 + 1]) + 1
        dp[i] %= M

    print(dp[-1] % M)
N = int(input())
S = input()
M = 10 ** 9 + 7

# dp[i, j] = number of permutation of (1, 2, ..., i) that satisfies s[:i - 1] and ends with j
dp = [[0 for _ in range(N + 1)] for _ in range(N + 1)]
dp[1][1] = 1

for i in range(2, N + 1):
    
    pref = [0 for _ in range(N + 1)]
    pref[0] = dp[i - 1][0]
    for j in range(1, N + 1):
        pref[j] = (pref[j - 1] + dp[i - 1][j]) % M

    for j in range(1, N + 1):
        if S[i - 2] == '<':
            dp[i][j] = pref[j - 1]
        else:
            dp[i][j] = (pref[i - 1] - pref[j - 1] + M) % M

    # for j in range(1, N + 1):
    #     if S[i - 2] == '<':
    #         for k in range(1, j):
    #             dp[i][j] += dp[i - 1][k]
    #             dp[i][j] %= M
    #     else:
    #         for k in range(j, i):
    #             dp[i][j] += dp[i - 1][k]
    #             dp[i][j] %= M

print(sum(dp[N][j] for j in range(1, N + 1)) % M)

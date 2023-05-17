S = input()[::-1]
M = 10 ** 9 + 7
N = len(S)

# dp[i, r] = number of valid ways to fill x[i - 1]...x[0] and x[i - 1]...x[0] % 13 = r

dp = [[0 for _ in range(13)] for _ in range(N + 1)]
dp[0][0] = 1  # empty string

pow10 = [1]
for i in range(1, N):
    pow10.append(pow10[-1] * 10 % 13)

for i in range(N):
    for r in range(13):  # x[i - 1]
        if S[i] != '?':
            d = int(S[i])
            new_r = (d * pow10[i] + r) % 13
            dp[i + 1][new_r] += dp[i][r]
            dp[i + 1][new_r] %= M
        else:
            for d in range(10):  # x[i]
                new_r = (d * pow10[i] + r) % 13
                dp[i + 1][new_r] += dp[i][r]
                dp[i + 1][new_r] %= M

print(dp[N][5])
K = [int(d) for d in input()]
N = len(K)
D = int(input())
M = 10 ** 9 + 7

dp = [[[0 for _ in range(2)] for _ in range(D)] for _ in range(N + 1)]

dp[N][0][0] = 1
dp[N][0][1] = 1

for i in range(N - 1, -1, -1):
    for rem in range(D):
        for is_pref in range(2):
            for d in range(K[i] + 1 if is_pref == 1 else 10):
                dp[i][rem][is_pref] += dp[i + 1][(rem + d) % D][is_pref and d == K[i]]
                dp[i][rem][is_pref] %= M

print((dp[0][0][1] - 1 + M) % M)

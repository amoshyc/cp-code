K = [int(x) for x in reversed(input())]
N = len(K)
D = int(input())
M = 10 ** 9 + 7

# dp[i, s, f] = #ways to fill x[i - 1]...x[1]x[0] such that
#   1. digit sum mod D = s
#   2. x[i - 1]...x[1]x[0] <= k[i - 1]...k[1]k[0] (f = 0) or not (f = 1)
# By filling the digit x[i] with d (d = 0, 1, ..., 9):
#   dp[i, s, f] -> dp[i + 1, (s + d) % D, f']
dp = [[[0, 0] for _ in range(D)] for _ in range(N + 1)]
dp[0][0][0] = 1

for i in range(N):
    for s in range(D):
        for f in range(2):
            for d in range(10):
                new_s = (s + d) % D
                new_f = int(d > K[i] or (d == K[i] and f == 1))
                dp[i + 1][new_s][new_f] += dp[i][s][f]
                dp[i + 1][new_s][new_f] %= M

print((dp[N][0][0] - 1 + M) % M) # subtract the 0
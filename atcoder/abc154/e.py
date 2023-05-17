K = input()
M = int(input())
N = len(K)
k = [int(x) for x in reversed(K)]

dp = [[[0, 0] for _ in range(4)] for _ in range(N + 1)]
dp[0][0][0] = 1 # empty string
for i in range(N):
    for c in range(4): # x[i - 1]
        for f in range(2):
            for d in range(10): # x[i]
                new_c = c + int(d > 0)
                new_f = int(d > k[i] or (d == k[i] and f == 1))
                if new_c <= 3:
                    dp[i + 1][new_c][new_f] += dp[i][c][f]

print(dp[N][M][0])
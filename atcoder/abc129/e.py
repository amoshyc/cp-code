'''
dp[i, f] = number of valid (a[i - 1]a[i - 2]...a[0], b[i - 1]b[i - 2]...b[0])
    and
    (f = 0) a[i - 1]a[i - 2]...a[0] + b[i - 1]b[i - 2]...b[0] <= l[i - 1]l[i - 2]...l[0]
    (f = 1) a[i - 1]a[i - 2]...a[0] + b[i - 1]b[i - 2]...b[0] > l[i - 1]l[i - 2]...l[0]

Answer is dp[N][0].

Enuermate a[i] and b[i],

a[i] = 0 and b[i] = 0:
a[i] = 1 and b[i] = 0:
a[i] = 0 and b[i] = 1:
    dp[i, f] -> dp[i + 1][f']
'''
mod = 10 ** 9 + 7
L = [int(c) for c in input()[::-1]]
N = len(L)

dp = [[0, 0] for _ in range(N + 1)]  # shaped [N + 1, 2]
dp[0][0] = 1

for i in range(N):
    for f in range(2):
        # a[i] = 0, b[i] = 0
        new_f = (L[i] == 0 and f == 1)
        dp[i + 1][new_f] += dp[i][f]
        dp[i + 1][new_f] %= mod

        # a[i] = 1, b[i] = 0
        new_f = (L[i] == 0) or (L[i] == 1 and f == 1)
        dp[i + 1][new_f] += dp[i][f]
        dp[i + 1][new_f] %= mod

        # a[i] = 0, b[i] = 1
        new_f = (L[i] == 0) or (L[i] == 1 and f == 1)
        dp[i + 1][new_f] += dp[i][f]
        dp[i + 1][new_f] %= mod

print(dp[N][0])
n, k = map(int, input().split())
s = input()

def judge(a, b):
    if (a, b) in [('R', 'S'), ('P', 'R'), ('S', 'P')]:
        return a
    else:
        return b

# dp[i, r] = fav hand of winner of [r, r + 2**i)
dp = [[None for _ in range(n)] for _ in range(k + 1)]
for r in range(n):
    dp[0][r] = s[r % n]
for i in range(1, k + 1):
    for r in range(n):
        dp[i][r] = judge(dp[i - 1][r], dp[i - 1][(r + 2 ** (i - 1)) % n])
print(dp[k][0])
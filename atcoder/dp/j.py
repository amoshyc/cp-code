# https://discuss.codechef.com/t/how-to-solve-this-problem/50519/5

import sys
sys.setrecursionlimit(10 ** 9)

def E(a, b, c):
    if dp[a][b][c] > -0.5:
        return dp[a][b][c]

    if a == 0 and b == 0 and c == 0:
        dp[0][0][0] = 0
        return 0
    
    ret = 1
    if a >= 1:
        ret += a / N * E(a - 1, b , c)
    if b >= 1:
        ret += b / N * E(a + 1, b - 1, c)
    if c >= 1:
        ret += c / N * E(a, b + 1, c - 1)
    ret /= (a + b + c) / N

    dp[a][b][c] = ret
    
    return ret

N = int(input())
M = 300 + 10
cnt = [0, 0, 0, 0]
for x in input().split():
    cnt[int(x)] += 1

dp = [[[-1.0 for _ in range(M)] for _ in range(M)] for _ in range(M)]
print('{:.15f}'.format(E(cnt[1], cnt[2], cnt[3])))
N, K = map(int, input().split())
A = [int(x) for x in input().split()]

# dp[i][j] = True if Taro will win else False when
#   i stones remining and
#   it is (Taro if j == 0 else Jiro)'s turn
dp = [[-1, -1] for i in range(K + 1)]
dp[0][0] = False
dp[0][1] = False
for i in range(1, K + 1):
    dp[i][0] = True if any(dp[i - a][1] is False for a in A if i - a >= 0) else False
    dp[i][1] = True if any(dp[i - a][0] is False for a in A if i - a >= 0) else False

# from tabulate import tabulate
# print(tabulate(dp, showindex=True, headers=['Taro', 'Jiro']))
print('First' if dp[K][0] else 'Second')

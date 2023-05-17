N, K = map(int, input().split())
A = [int(x) for x in input().split()]

# dp[x][t] = number of stones Takahashi can get when the piles is x and it's Takahashi's turn (t = 0).
# dp[x][0] = max(dp[x - a][1] + a for a in A if x >= a)
# dp[x][1] = min(dp[x - a][0] for a in A if x >= a)

dp = [[0, 0] for _ in range(N + 1)]
dp[0][0] = 0
dp[0][1] = 0
dp[1][0] = 1
dp[1][1] = 0

for x in range(1, N + 1):
    dp[x][0] = max(dp[x - a][1] + a for a in A if x >= a)
    dp[x][1] = min(dp[x - a][0] for a in A if x >= a)
print(dp[N][0])

# def f(x, turn):
#     if turn == 0:
#         if x <= 1:
#             return x
#         return max(f(x - a, 1 - turn) + a for a in A if x >= a)
#     else:
#         if x <= 1:
#             return 0
#         return min(f(x - a, 1 - turn) for a in A if x >= a)
# print(f(N, 0))
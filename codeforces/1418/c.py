'''
dp[i, j] = the min skip needed to kill first i + 1 bosses and i-th boss is killed by j
    j = 0: friend
    j = 1: me

Enumerate the number of bosses killed in last session (friend)
dp[i, 0] = min(dp[i - 1, 1] + A[i], dp[i - 2, 1] + A[i - 1] + A[i])

Enumerate the number of bosses killed in last session (me)
dp[i, 1] = min(dp[i - 1, 0], dp[i - 2, 0])
'''

def solve():
    N = int(input())
    A = [int(x) for x in input().split()]

    dp = [[0, 0] for _ in range(N)]

    if N == 1:
        return A[0]
    if N == 2:
        return min(A[0], A[0] + A[1])

    dp[0][0] = A[0]
    dp[1][0] = A[0] + A[1]
    dp[0][1] = 10**10 # impossible
    dp[1][1] = A[0]

    for i in range(2, N):
        dp[i][0] = min(dp[i - 1][1] + A[i], dp[i - 2][1] + A[i - 1] + A[i])
        dp[i][1] = min(dp[i - 1][0], dp[i - 2][0])
    
    return min(dp[N - 1])

tc = int(input())
for _ in range(tc):
    print(solve())

N, S = map(int, input().split())
A = []
for _ in range(N):
    x, y = map(int, input().split())
    A.append((x, y))

dp = [[False for _ in range(10000 + 1)] for _ in range(N)]
dp[0][A[0][0]] = True
dp[0][A[0][1]] = True

chose = dict()

for i in range(1, N):
    for j in range(S + 1):
        if j - A[i][0] >= 0 and dp[i - 1][j - A[i][0]]:
            dp[i][j] = True
            chose[(i, j)] = 0
        if j - A[i][1] >= 0 and dp[i - 1][j - A[i][1]]:
            dp[i][j] = True
            chose[(i, j)] = 1

# print(chose)

if dp[N - 1][S]:
    print('Yes')
    result = []
    j = S
    for i in range(N - 1, 0, -1):
        # print(i, j)
        if chose[(i, j)] == 0:
            result.append('H')
            j = j - A[i][0]
        else:
            result.append('T')
            j = j - A[i][1]
    if j == A[0][0]:
        result.append('H')
    else:
        result.append('T')
    print(''.join(reversed(result)))
else:
    print('No')

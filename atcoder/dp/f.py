S = '#' + input()
T = '$' + input()

dp = [[0 for _ in range(len(T))] for _ in range(len(S))]
prev = [[-1 for _ in range(len(T))] for _ in range(len(S))]

for i in range(1, len(S)):
    for j in range(1, len(T)):
        if S[i] == T[j]:
            dp[i][j] = dp[i - 1][j - 1] + 1
            prev[i][j] = 0
        elif dp[i - 1][j] > dp[i][j - 1]:
            dp[i][j] = dp[i - 1][j]
            prev[i][j] = 1
        else:
            dp[i][j] = dp[i][j - 1]
            prev[i][j] = 2

lcs = []
i, j = len(S) - 1, len(T) - 1
while i >= 0 and j >= 0:
    if prev[i][j] == 0:
        lcs.append(S[i])
        i -= 1
        j -= 1
    elif prev[i][j] == 1:
        i -= 1
    elif prev[i][j] == 2:
        j -= 1
    else:
        break
print(''.join(map(str, reversed(lcs))))
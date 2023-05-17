N = int(input())
A = [int(x) for x in input().split()]


# dp[i] = minimum sum of substring ending at A[i]
dp = [0] * N
dp[0] = A[0]
for i in range(1, N):
    dp[i] = min(dp[i - 1] + A[i], A[i])

pos = 0
add = 0
ans = 0
for i in range(N):
    add += A[i]
    pos += add
    ans = max(ans, pos - dp[i])
    ans = max(ans, pos)
print(ans)

# pos = 0
# ans2 = 0
# for i in range(N):
#     print(i, pos)
#     for j in range(i + 1):
#         pos = pos + A[j]
#         print('\t', pos)
#         ans2 = max(ans2, pos)
# print(ans2)

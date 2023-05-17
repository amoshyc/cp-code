N = int(input())
M = 10**9 + 7

ans = pow(10, N, M)
ans = ans + M - pow(9, N, M)
ans = ans + M - pow(9, N, M)
ans = ans + pow(8, N, M)
ans = ans % M
print(ans)
mod = 10**9 + 7
N = int(input())
fact = [1] * (N + 1)
for i in range(1, N + 1):
    fact[i] = fact[i - 1] * i % mod
ans = fact[N]
ans = ans - pow(2, N - 1, mod) + mod
ans = ans % mod
print(ans)
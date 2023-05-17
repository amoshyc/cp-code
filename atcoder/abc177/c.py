mod = 10 ** 9 + 7

N = int(input())
A = list(map(int, input().split()))

ans = 0
pref = A[0]
for i in range(1, N):
    ans = (ans + A[i] * pref % mod) % mod
    pref = (pref + A[i]) % mod
print(ans)

mod = int(1e9 + 7)

N, K = map(int, input().split())
A = list(map(int, input().split()))

ans = 0
for i in range(N):
    cnt1 = len([x for x in A[i + 1:] if x < A[i]])
    mul1 = K * (K + 1) // 2 % mod
    cnt2 = len([x for x in A[:i] if x < A[i]])
    mul2 = (K - 1) * K // 2 % mod
    ans = (ans + cnt1 * mul1 % mod) % mod
    ans = (ans + cnt2 * mul2 % mod) % mod
print(ans)
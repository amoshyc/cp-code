def minv(a, m):
    b = m
    p, q = 1, 0
    while b:
        c, r = divmod(a, b)
        a, b = b, r
        p, q = q, p - c * q
    return p % m


N = int(input())
A = [int(x) for x in input().split()]
M = 998244353

E = [0 for _ in range(N)]
S = [0 for _ in range(N + 1)] # suffix
E[N - 1] = 0
S[N - 1] = E[N - 1]

for i in range(N - 2, -1, -1):
    E[i] = (1 + A[i] + S[i + 1] - S[i + A[i] + 1]) % M * minv(A[i], M) % M
    S[i] = (S[i + 1] + E[i]) % M
print(E[0])
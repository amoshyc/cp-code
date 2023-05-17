mod = 998244353
N = int(input())
A = [int(x) for x in input().split()]
B = [int(x) for x in input().split()]

prev = [0 for _ in range(3000 + 1)]
for c in range(A[0], B[0] + 1):
    prev[c] = 1
for i in range(1, N):
    curr = [0 for _ in range(3000 + 1)]
    S = sum(prev[: A[i]]) % mod
    for c in range(A[i], B[i] + 1):
        S += prev[c]
        S %= mod
        curr[c] = S
    prev = curr
print(sum(curr[A[i] : B[i] + 1]) % mod)

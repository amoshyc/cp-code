N, K, X = map(int, input().split())
A = [int(x) for x in input().split()]

for i in range(N):
    if A[i] < X:
        continue
    n = A[i] // X
    n = min(n, K)
    A[i] -= n * X
    K -= n

A = sorted(A, reverse=True)
for i in range(N):
    if A[i] > 0 and K > 0:
        A[i] = 0
        K -= 1

print(sum(A))
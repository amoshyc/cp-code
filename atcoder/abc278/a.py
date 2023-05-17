N, K = map(int, input().split())
A = [int(x) for x in input().split()]
for _ in range(K):
    A = A[1:] + [0]
print(' '.join(map(str, A)))
N, Q = map(int, input().split())

P = list(range(N))
A = list(range(N))

for _ in range(Q):
    x = int(input()) - 1
    p = P[x]
    if p + 1 == N: # righmost
        y = A[p - 1]
        A[p], A[p - 1] = A[p - 1], A[p]
        P[y] = p
        P[x] = p - 1
    else:
        y = A[p + 1]
        A[p], A[p + 1] = A[p + 1], A[p]
        P[y] = p
        P[x] = p + 1

print(' '.join([f'{a + 1}' for a in A]))
N, Q = map(int, input().split())

A = []
for _ in range(N):
    xs = [int(x) for x in input().split()]
    A.append(xs[1:])

for _ in range(Q):
    s, t = map(int, input().split())
    print(A[s - 1][t - 1])
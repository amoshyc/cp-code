N, K, X = map(int, input().split())
A = [int(x) for x in input().split()]

A.sort()
A.reverse()


def ok(m):
    # drink first m cups and first N - K of them is water
    sake = sum(A[N - K : m])
    return sake >= X


# 0 0 0 1 1 1
lb, ub = 0, N + 1
if not ok(ub):
    print(-1)
    exit()

while ub - lb > 1:
    m = (lb + ub) // 2
    if ok(m):
        ub = m
    else:
        lb = m
print(ub)

N, K = map(int, input().split())
A, B = [], []
for _ in range(N):
    a, b = map(int, input().split())
    A.append(a)
    B.append(b)

# 1 1 1 0 0 0
def ok(m):
    return sum(b for a, b in zip(A, B) if a >= m) > K

# real range is [1, max(A) + 1]
lb, ub = 0, max(A) + 10
while ub - lb > 1:
    m = (lb + ub) // 2
    if ok(m):
        lb = m
    else:
        ub = m

print(ub)
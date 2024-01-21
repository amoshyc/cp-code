def ceil_div(a, b):
    return (a + b - 1) // b

N, K = map(int, input().split())
A = list(map(int, input().split()))


def check(l):
    cnt = sum(ceil_div(a, l) - 1 for a in A)
    return cnt <= K

lb, ub = 0, max(A)
for _ in range(50):
    mid = (lb + ub) / 2
    if check(mid):
        ub = mid
    else:
        lb = mid

from math import ceil
print(ceil(ub))
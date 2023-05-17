from math import ceil

N, K = map(int, input().split())
A = list(map(int, input().split()))


def check(l):
    cnt = sum(ceil(a / l) - 1 for a in A)
    return cnt <= K

lb, ub = 0, max(A)
for _ in range(50):
    mid = (lb + ub) / 2
    if check(mid):
        ub = mid
    else:
        lb = mid
print(ceil(ub))
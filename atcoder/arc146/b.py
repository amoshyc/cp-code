N, M, K = map(int, input().split())
A = [int(x) for x in input().split()]


def super_value(a, mask):
    """Find the minimum b, such that
    1. b >= a
    2. (b & mask) = b
    """
    for i in range(32, -1, -1):
        if ((mask >> i) & 1) == 1 and ((a >> i) & 1) == 0:
            return ((a >> i) << i) + (mask & ((1 << (i + 1)) - 1))
    return a


def check(x):
    needs = [super_value(a, x) - a for a in A]
    needs = sorted(needs)
    return sum(needs[:K]) <= M


lb, ub = 0, 2**33
while ub - lb > 1:
    m = (lb + ub) // 2
    if check(m):
        lb = m
    else:
        ub = m
print(lb)

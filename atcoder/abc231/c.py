N, Q = map(int, input().split())

A = [int(x) for x in input().split()]
A.sort()

for _ in range(Q):
    x = int(input())

    def f(m):
        return A[m] >= x

    # 0 0 0 1 1 1
    lb, ub = 0, N - 1
    if f(lb) is True:
        print(N)
        continue
    if f(ub) is False:
        print(0)
        continue
    while ub - lb > 1:
        m = (lb + ub) // 2
        if f(m) is True:
            ub = m
        else:
            lb = m
    print(N - ub)

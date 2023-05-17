def solve():
    N = int(input())
    A = [int(x) for x in input().split()]

    def check(x):
        owned = set()
        supply = 0
        for a in A:
            if a <= x and a not in owned:
                owned.add(a)
            else:
                supply += 1
        need = sum(1 for i in range(1, x + 1) if i not in owned)
        return need <= supply // 2

    # 1 1 1 0 0 0
    lb, ub = 0, N
    if check(lb) is False:
        return lb
    if check(ub) is True:
        return ub
    while ub - lb > 1:
        m = (lb + ub) // 2
        if check(m):
            lb = m
        else:
            ub = m
    return lb


print(solve())

def solve():
    N = int(input())

    lb, ub = 0, 0
    for n_digit in range(1, 100):
        lb = ub + 1
        ub = (ub + 1) * 26
        if lb <= N <= ub:
            off = N - lb
            res = []
            while off > 0:
                off, r = divmod(off, 26)
                res.append(r)
            while len(res) < n_digit:
                res.append(0)
            res.reverse()
            res = ''.join([chr(ord('a') + x) for x in res])
            return res
    
print(solve())
def f(a, b):
    return a ** 3 + a ** 2 * b + a * b ** 2 + b ** 3

N = int(input())
ans = 10 ** 19
for a in range(10 ** 6 + 1):
    if a ** 3 > N:
        break
    
    def check(m):
        return f(a, m) >= N

    # 0 0 0 0 1 1 1
    lb = -1
    ub = 10 ** 6 + 1
    while ub - lb > 1:
        m = (lb + ub) // 2
        if check(m):
            ub = m
        else:
            lb = m
    b = ub
    ans = min(ans, f(a, b))
print(ans)
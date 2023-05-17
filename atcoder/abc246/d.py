def f(a, b):
    return a ** 3 + a ** 2 * b + a * b ** 2 + b ** 3

N = int(input())
ans = 10 ** 19
for a in range(10 ** 6 + 1):
    if a > N:
        break
    # a ** 3 + a ** 2 * b + a * b ** 2 + b ** 3 >= N
    # b(a ** 2 + b + 1) = N - a ** 3
    lb = -1
    ub = 10 ** 6 + 1
    # 0 0 0 0 1 1 1
    while ub - lb > 1:
        m = (lb + ub) // 2
        if f(a, m) >= N:
            ub = m
        else:
            lb = m
    b = ub
    ans = min(ans, f(a, b))
print(ans)
            
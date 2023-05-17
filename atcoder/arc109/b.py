from math import sqrt, floor

# max x such that x * (x + 1) / 2 <= n + 1
n = int(input())
# x = floor((sqrt(8 * n + 8 + 1) - 1) / 2)
lb, ub = 0, n + 1
while ub - lb > 1:
    m = (lb + ub) // 2
    if (m + 1) * m // 2 <= n + 1:
        lb = m
    else:
        ub = m
x = lb
print(n - x + 1)
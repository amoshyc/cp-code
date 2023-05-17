N = int(input())

lb, ub = 0, N
while ub - lb > 1:
    m = (lb + ub) // 2
    print('?', 1, m, 1, N, flush=True)
    a = int(input())
    if a < m:
        ub = m
    else:
        lb = m

row = ub

lb, ub = 0, N
while ub - lb > 1:
    m = (lb + ub) // 2
    print('?', 1, N, 1, m, flush=True)
    a = int(input())
    if a < m:
        ub = m
    else:
        lb = m

col = ub

print('!', row, col, flush=True)
P = [int(x) for x in input().split()]
S = sum(P)

for mask in range(1, 1 << 4):
    s = sum(x for i, x in enumerate(P) if mask & (1 << i))
    if s == S - s:
        print('Yes')
        break
else:
    print('No')
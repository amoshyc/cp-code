N = int(input())

indices = [i for i in range(62) if N & (1 << i)]
M = len(indices)

res = []
for mask in range(1 << M):
    x = N
    for i in range(62):
        if (mask & (1 << i)) and (x & (1 << indices[i])):
            x ^= (1 << indices[i])
    res.append(x)
res.sort()

print('\n'.join(map(str, res)))
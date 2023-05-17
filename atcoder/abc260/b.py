N, X, Y, Z = map(int, input().split())
A = [int(x) for x in input().split()]
B = [int(x) for x in input().split()]

indices = list(range(N))
ans = []

indices = sorted(indices, key=lambda i: (-A[i], i))
ans.extend(indices[:X])
indices = indices[X:]

indices = sorted(indices, key=lambda i: (-B[i], i))
ans.extend(indices[:Y])
indices = indices[Y:]

indices = sorted(indices, key=lambda i: (-(A[i] + B[i]), i))
ans.extend(indices[:Z])
indices = indices[Z:]

ans = sorted(ans)
print('\n'.join([str(x + 1) for x in ans]))

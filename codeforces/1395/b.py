N, M, X, Y = map(int, input().split())

paths = []
for r in range(N):
    if r % 2 == 0:
        for c in range(1, M):
            paths.append((r, c))
    else:
        for c in range(M - 1, 0, -1):
            paths.append((r, c))
for r in range(N - 1, -1, -1):
    paths.append((r, 0))

m = paths.index((X - 1, Y - 1))
print('\n'.join(['{} {}'.format(r + 1, c + 1) for r, c in paths[m:]]))
print('\n'.join(['{} {}'.format(r + 1, c + 1) for r, c in paths[:m]]))
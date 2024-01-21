N = int(input())
xs, ys = [], []
for _ in range(N):
    x, y = map(int, input().split())
    xs.append(x)
    ys.append(y)
U = [x + y for x, y in zip(xs, ys)]
V = [x - y for x, y in zip(xs, ys)]
ans = max([max(U) - min(U), max(V) - min(V)])
print(ans)
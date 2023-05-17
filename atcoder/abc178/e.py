N = int(input())
xs, ys = [], []
for _ in range(N):
    x, y = map(int, input().split())
    xs.append(x)
    ys.append(y)
A = [x + y for x, y in zip(xs, ys)]
B = [x - y for x, y in zip(xs, ys)]
ans = max([max(A) - min(A), max(B) - min(B)])
print(ans)
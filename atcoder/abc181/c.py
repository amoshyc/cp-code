N = int(input())
xs, ys = [], []
for _ in range(N):
    x, y = map(int, input().split())
    xs.append(x)
    ys.append(y)

for i in range(N):
    for j in range(i + 1, N):
        for k in range(j + 1, N):
            # (ys[j] - ys[i]) / (xs[j] - xs[i]) == (ys[k] - ys[j]) / (xs[k] - xs[j])
            ij_dx, ij_dy = xs[j] - xs[i], ys[j] - ys[i]
            jk_dx, jk_dy = xs[k] - xs[j], ys[k] - ys[j]
            if ij_dy * jk_dx == ij_dx * jk_dy:
                print('Yes')
                exit()

print('No')
N = int(input())
X, Y = [], []
for _ in range(N):
    x, y = map(int, input().split())
    X.append(x)
    Y.append(y)

cnt = 0
for i in range(N):
    for j in range(i + 1, N):
        for k in range(j + 1, N):
            area = abs(
                X[i] * Y[j]
                + X[j] * Y[k]
                + X[k] * Y[i]
                - X[j] * Y[i]
                - X[k] * Y[j]
                - X[i] * Y[k]
            )
            if area > 0:
                cnt += 1

print(cnt)

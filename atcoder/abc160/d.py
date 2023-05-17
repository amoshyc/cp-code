N, X, Y = map(int, input().split())
X, Y = X - 1, Y - 1

cnt = [0] * N
for i in range(N - 1):
    for j in range(i + 1, N):
        dist = min(j - i, abs(j - Y) + abs(X - i) + 1)
        cnt[dist] += 1

print('\n'.join(map(str, cnt[1:])))


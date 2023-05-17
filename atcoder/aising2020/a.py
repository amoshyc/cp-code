L, R, d = map(int, input().split())

cnt = 0
for i in range(1, 150):
    if L <= i * d <= R:
        cnt += 1
print(cnt)
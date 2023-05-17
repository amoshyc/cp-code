M, D = map(int, input().split())
cnt = 0
for m in range(1, M + 1):
    for d in range(1, D + 1):
        d1, d2 = divmod(d, 10)
        if d1 >= 2 and d2 >= 2 and d1 * d2 == m:
            cnt += 1
print(cnt)
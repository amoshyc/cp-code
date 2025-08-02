X, Y = map(int, input().split())

cnt = 0
for a in range(1, 7):
    for b in range(1, 7):
        if a + b >= X or abs(a - b) >= Y:
            cnt += 1
print(f'{cnt / 6 / 6:.10f}')
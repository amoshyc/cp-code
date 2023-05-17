A, B = map(int, input().split())

for p in range(1, 10000):
    if p * 8 // 100 == A and p // 10 == B:
        print(p)
        break
else:
    print(-1)
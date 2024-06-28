TC = int(input())

for _ in range(TC):
    N = int(input())
    ans = (N + 1) // 2
    if N % 2 == 0:
        ans += 1
    print(ans)
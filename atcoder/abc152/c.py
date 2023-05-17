N = int(input())
P = list(map(int, input().split()))

mn = N + 1
ans = 0
for p in P:
    if p < mn:
        ans += 1
    mn = min(mn, p)
print(ans)
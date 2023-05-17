N, M, T = map(int, input().split())
power = N
ans = 'Yes'
prev = 0
for _ in range(M):
    a, b = map(int, input().split())
    power = power - (a - prev)
    if power <= 0:
        ans = 'No'
    power = min(N, power + (b - a))
    prev = b
power = power - (T - prev)
if power <= 0:
    ans = 'No'
print(ans)

N = int(input())
a = list(map(int, input().split()))
ans = 0
for i in range(1, N - 1):
    if a[i] < a[i - 1] and a[i] < a[i + 1]:
        ans += 1
    if a[i] > a[i - 1] and a[i] > a[i + 1]:
        ans += 1
print(ans)
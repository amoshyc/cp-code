from collections import defaultdict

L = int(input())
A = list(map(int, input().split()))

cnt = defaultdict(int)
for i, a in enumerate(A):
    cnt[a] += 1

res = 0
for val, num in cnt.items():
    res += num * (num - 1) // 2

for a in A:
    ans = res
    num = cnt[a]
    ans -= num * (num - 1) // 2
    ans += max((num - 1) * (num - 2) // 2, 0)
    print(ans)
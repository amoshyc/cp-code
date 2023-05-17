N = int(input())
A = sorted([int(x) for x in input().split()])
cum = 0
ans = 0
for i, a in enumerate(A):
    ans += i * a - cum
    cum += a
print(ans)
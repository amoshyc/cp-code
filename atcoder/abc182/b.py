N = int(input())
A = [int(x) for x in input().split()]

ans = -1
cnt = -1
for x in range(2, 1000 + 1):
    val = sum(1 for a in A if a % x == 0)
    if val > cnt:
        ans = x
        cnt = val
print(ans)
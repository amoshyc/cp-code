N = int(input())
ans = set()
for _ in range(N):
    _, *a = [int(x) for x in input().split()]
    ans.add(tuple(a))
print(len(ans))
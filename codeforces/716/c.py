N = int(input())

ans = {}
ans[1] = 14

num = 4
for level in range(2, N + 1):
    val = (level * (level + 1)) ** 2
    ans[level] = (val - num) // level
    num = (level * (level + 1))

for i in range(1, N + 1):
    print(ans[i])

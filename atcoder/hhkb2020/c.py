N = int(input())
P = [int(x) for x in input().split()]

vis = [False for _ in range(200000 + 10)]
val = 0

for p in P:
    vis[p] = True
    while vis[val]:
        val += 1
    print(val)
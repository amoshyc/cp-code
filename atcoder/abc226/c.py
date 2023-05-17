N = int(input())
T = []
G = [[] for _ in range(N)]
for u in range(N):
    t, k, *a = [int(x) for x in input().split()]
    T.append(t)
    for v in a:
        v = v - 1
        G[u].append(v)

stack = [N - 1]
ans = 0
vis = [False for _ in range(N)]
vis[N - 1] = True
while len(stack) > 0:
    u = stack.pop()
    ans += T[u]
    for v in G[u]:
        if not vis[v]:
            stack.append(v)
            vis[v] = True
print(ans)

N, M = map(int, input().split())
mod = 998244353
G = [[] for _ in range(N)]
for _ in range(M):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    G[u].append(v)
    G[v].append(u)

ans = 1
vis = [False for _ in range(N)]

for root in range(N):
    if vis[root]:
        continue

    vis[root] = True
    stack = [(root, -1)]
    edges = set()
    verts = set()
    while len(stack) > 0:
        u, p = stack.pop()
        verts.add(u)
        for v in G[u]:
            edges.add((min(u, v), max(u, v)))
        for v in G[u]:
            if v != p and not vis[v]:
                vis[v] = True
                stack.append((v, u))
            
    if len(edges) != len(verts):
        ans = 0
        break
    else:
        ans = ans * 2 % mod

print(ans)
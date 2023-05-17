from collections import deque


def bfs(G, root, par):
    parent = {root: par}
    depth = {root: 0}
    nodes = []
    queue = deque([(root, par)])
    while len(queue) > 0:
        u, p = queue.popleft()
        nodes.append(u)
        for v in G[u]:
            if v != p:
                parent[v] = u
                depth[v] = depth[u] + 1
                queue.append((v, u))
    return nodes, parent, depth


mod = 998244353
N = int(input())
G = [[] for _ in range(N)]
for _ in range(N - 1):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    G[u].append(v)
    G[v].append(u)

nodes, parent, _ = bfs(G, 0, -1)
s = nodes[-1]
nodes, parent, _ = bfs(G, s, -1)
t = nodes[-1]
path = [t]
while parent[path[-1]] != -1:
    path.append(parent[path[-1]])
diameter = len(path) - 1

if diameter % 2 == 0:  # centered tree
    root = path[diameter // 2]
    cnts = []
    for child in G[root]:
        nodes, _, depth = bfs(G, child, root)
        cnt = sum(1 for u in nodes if depth[u] == diameter // 2 - 1)
        cnts.append(cnt)
    ans = 1
    for cnt in cnts:
        ans = ans * (cnt + 1) % mod
    for cnt in cnts:
        ans = ans - cnt + mod
    ans = ans - 1 + mod
else:  # bi-centered tree
    root1, root2 = path[diameter // 2], path[diameter // 2 + 1]
    nodes, _, depth = bfs(G, root1, root2)
    cnt1 = sum(1 for u in nodes if depth[u] == (diameter - 1) // 2)
    nodes, _, depth = bfs(G, root2, root1)
    cnt2 = sum(1 for u in nodes if depth[u] == (diameter - 1) // 2)
    ans = cnt1 * cnt2 % mod

print(ans % mod)

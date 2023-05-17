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


N, X, Y = map(int, input().split())
X, Y = X - 1, Y - 1
G = [[] for _ in range(N)]
for _ in range(N - 1):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    G[u].append(v)
    G[v].append(u)


vis, par, dep = bfs(G, X, -1)

paths = []
while Y != -1:
    paths.append(Y)
    Y = par[Y]

print(' '.join([f'{u + 1}' for u in reversed(paths)]))
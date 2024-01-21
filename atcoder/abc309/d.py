from collections import deque

N1, N2, M = map(int, input().split())
adj1 = [[] for _ in range(N1)]
adj2 = [[] for _ in range(N2)]
for _ in range(M):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    if u < N1:
        adj1[u].append(v)
        adj1[v].append(u)
    else:
        u, v = u - N1, v - N1
        adj2[u].append(v)
        adj2[v].append(u)


def bfs(adj, root, inf=10**18):
    n = len(adj)
    que = deque()
    dis = [inf for _ in range(n)]

    que.append(root)
    dis[root] = 0

    while len(que) > 0:
        u = que.popleft()
        for v in adj[u]:
            if dis[u] + 1 < dis[v]:
                dis[v] = dis[u] + 1
                que.append(v)

    return max(dis)

d1 = bfs(adj1, 0)
d2 = bfs(adj2, N2 - 1)
print(d1 + d2 + 1)
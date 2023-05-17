import math
from collections import deque


def bfs01(adj, source, inf=math.inf):
    dis = [inf for _ in range(len(adj))]
    dis[source] = 0
    que = deque([source])
    while len(que) > 0:
        u = que.popleft()
        for (v, w) in adj[u]:
            if dis[v] == inf:
                dis[v] = dis[u] + w
                if w == 1:
                    que.append(v)
                else:
                    que.appendleft(v)
    return dis


N, M, K = map(int, input().split())
G = [[] for _ in range(2 * N)]
for _ in range(M):
    u, v, a = map(int, input().split())
    u, v = u - 1, v - 1
    if a == 1:
        G[u].append((v, 1))
        G[v].append((u, 1))
    else:
        G[u + N].append((v + N, 1))
        G[v + N].append((u + N, 1))
for s in map(int, input().split()):
    s = s - 1
    G[s].append((s + N, 0))
    G[s + N].append((s, 0))

dis = bfs01(G, 0)
ans = min(dis[N - 1], dis[N - 1 + N])
if ans == math.inf:
    print(-1)
else:
    print(ans)

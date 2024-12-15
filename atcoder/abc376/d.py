from collections import deque

N, M = map(int, input().split())
adj = [[] for _ in range(N)]
for _ in range(M):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    adj[u].append(v)

# BFS from 0 -> dis[u] = distance from vertex 0
inf = 10**10
dis = [inf for _ in range(N)]
que = deque()

dis[0] = 0
que.append(0)

while len(que) > 0:
    u = que.popleft()
    for v in adj[u]:
        if dis[v] == inf:
            dis[v] = dis[u] + 1
            que.append(v)

# cycle = dis[u] + 1
ans = inf
for u in range(1, N):
    if 0 in adj[u]:
        ans = min(ans, dis[u] + 1)

if ans == inf:
    print(-1)
else:
    print(ans)

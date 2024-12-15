from collections import deque

N, D = map(int, input().split())
xs = []
ys = []
for _ in range(N):
    x, y = map(int, input().split())
    xs.append(x)
    ys.append(y)

adj = [[] for _ in range(N)]
for i in range(N):
    for j in range(i + 1, N):
        if (xs[j] - xs[i]) ** 2 + (ys[j] - ys[i]) ** 2 <= D**2:
            adj[i].append(j)
            adj[j].append(i)

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

ans = ["Yes" if dis[u] != inf else "No" for u in range(N)]
print("\n".join(ans))

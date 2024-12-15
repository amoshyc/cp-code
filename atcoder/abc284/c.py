from collections import deque

N, M = map(int, input().split())
adj = [[] for _ in range(N)]

for _ in range(M):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    adj[u].append(v)
    adj[v].append(u)

inf = 10**10
dis = [inf for _ in range(N)]
ans = 0

for root in range(N):
    if dis[root] == inf:
        # A new connected component
        ans += 1

        # BFS from root
        dis[root] = 0
        que = deque([root])

        while len(que) > 0:
            u = que.popleft()
            for v in adj[u]:
                if dis[v] == inf:
                    dis[v] = dis[u] + 1
                    que.append(v)

print(ans)

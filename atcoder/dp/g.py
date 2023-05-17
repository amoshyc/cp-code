from collections import deque


N, M = map(int, input().split())

in_deg = [0 for v in range(N)]
graph = [[] for v in range(N)]
for _ in range(M):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    graph[u].append(v)
    in_deg[v] += 1


# dp[v] = length of the longest path ending at v
dp = [0 for _ in range(N)]

# topological order
que = deque([v for v, deg in enumerate(in_deg) if deg == 0])
while len(que) > 0:
    u = que.popleft()
    for v in graph[u]:
        dp[v] = max(dp[v], dp[u] + 1)
        in_deg[v] -= 1
        if in_deg[v] == 0:
            que.append(v)

print(max(dp))

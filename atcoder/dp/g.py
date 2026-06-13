from collections import deque


def topological_sort(adj):
    N = len(adj)
    indeg = [0 for _ in range(N)]
    for u in range(N):
        for v in adj[u]:
            indeg[v] += 1

    que = deque([u for u in range(N) if indeg[u] == 0])

    sort = []
    while len(que) > 0:
        u = que.popleft()
        sort.append(u)
        for v in adj[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                que.append(v)

    return sort


N, M = map(int, input().split())
adj = [[] for _ in range(N)]
for _ in range(M):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    adj[u].append(v)

sort = topological_sort(adj)

# dp[u] = length of the longest path starting from u
# dp[u] = max(dp[v] for v in adj[u]) + 1
dp = [0 for _ in range(N)]
for u in reversed(sort):
    for v in adj[u]:
        dp[u] = max(dp[u], dp[v] + 1)

print(max(dp))

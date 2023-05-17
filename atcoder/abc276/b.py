N, M = map(int, input().split())
adj = [[] for _ in range(N)]
for _ in range(M):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    adj[u].append(v)
    adj[v].append(u)


for u in range(N):
    print(len(adj[u]), *[v + 1 for v in sorted(adj[u])])
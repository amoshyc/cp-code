import math

N, M, K = map(int, input().split())
edges = []
for _ in range(M):
    u, v, c = map(int, input().split())
    u, v = u - 1, v - 1
    edges.append((u, v, c))
E = [int(x) - 1 for x in input().split()]

d = [math.inf for _ in range(N)]
d[0] = 0
for eid in E:
    u, v, c = edges[eid]
    d[v] = min(d[v], d[u] + c)
print(d[-1] if d[-1] != math.inf else -1)
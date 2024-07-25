from heapq import heappush, heappop


def dijkstra(G, s, inf=10**18):
    dist = [inf for _ in range(len(G))]
    dist[s] = 0
    que = [(dist[s], s)]
    while len(que) > 0:
        d, u = heappop(que)
        if d > dist[u]:
            continue
        for v, w in G[u]:
            if dist[v] > dist[u] + w:
                dist[v] = dist[u] + w
                heappush(que, (dist[v], v))
    return dist


n, m = map(int, input().split())
arr = [int(x) for x in input().split()]
adj = [[] for _ in range(2 * n)]
for _ in range(m):
    u, v, w = map(int, input().split())
    u, v = u - 1, v - 1
    adj[2 * u + 1].append((2 * v + 0, w))
    adj[2 * v + 1].append((2 * u + 0, w))
for u in range(n):
    adj[2 * u + 0].append((2 * u + 1, arr[u]))

dis = dijkstra(adj, 0)
ans = dis[1::2][1:]
print(" ".join(map(str, ans)))

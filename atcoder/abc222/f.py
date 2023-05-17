from heapq import heappush, heappop


def dijkstra(G, s, inf=10 ** 18):
    dist = [inf for _ in range(len(G))]
    dist[s] = 0
    que = [(dist[s], s)]
    while len(que) > 0:
        d, u = heappop(que)
        if d > dist[u]:
            continue
        for (v, w) in G[u]:
            if dist[v] > dist[u] + w:
                dist[v] = dist[u] + w
                heappush(que, (dist[v], v))
    return dist

N = int(input())
G = [[] for _ in range(N)]
for _ in range(N - 1):
    a, b, c = map(int, input().split())
    a, b = a - 1, b - 1
    G[a].append((b, c))
    G[b].append((a, c))
D = [int(x) for x in input().split()]

dist = dijkstra(G, 0)
dist = [d1 + d2 for d1, d2 in zip(dist, D)]
max_dist = max(dist[u] for u in range(N) if u != 0)
s = next(u for u in range(N) if dist[u] == max_dist and u != 0)

dist = dijkstra(G, s)
dist = [d1 + d2 for d1, d2 in zip(dist, D)]
max_dist = max(dist[u] for u in range(N) if u != s)
t = next(u for u in range(N) if dist[u] == max_dist and u != s)

dist_s = dijkstra(G, s)
dist_t = dijkstra(G, t)
diameter = dist_s[t]
for u in range(N):
    if u == s:
        print(diameter + D[t])
    elif u == t:
        print(diameter + D[s])
    else:
        print(max(dist_s[u] + D[s], dist_t[u] + D[t]))

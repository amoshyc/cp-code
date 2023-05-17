from heapq import heappop, heappush


N, M = map(int, input().split())
G = [[] for _ in range(N)]
inD = [0 for _ in range(N)]
for _ in range(M):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    G[u].append(v)
    inD[v] += 1

que = [u for u in range(N) if inD[u] == 0]
ans = []
while len(que) > 0:
    u = heappop(que)
    ans.append(u + 1)
    for v in G[u]:
        inD[v] -= 1
        if inD[v] == 0:
            heappush(que, v)

if len(ans) == N:
    print(' '.join(map(str, ans)))
else:
    print(-1)



N = int(input())
deg = [0 for _ in range(N)]
for _ in range(N - 1):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    deg[u] += 1
    deg[v] += 1

if any(u for u in range(N) if deg[u] == (N - 1)):
    print('Yes')
else:
    print('No')
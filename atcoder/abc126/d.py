N = int(input())
G = [[] for _ in range(N)]
for _ in range(N - 1):
    u, v, w = map(int, input().split())
    u, v = u - 1, v - 1
    G[u].append((v, w))
    G[v].append((u, w))

color = [-1 for _ in range(N)]
color[0] = 0
stack = [(0, -1)]
while len(stack) > 0:
    u, p = stack.pop()
    for (v, w) in G[u]:
        if v != p:
            color[v] = (color[u] + w % 2) % 2
            stack.append((v, u))

print('\n'.join(map(str, color)))
N, M = map(int, input().split())
adjs = [[] for _ in range(N)]
for _ in range(M):
    u, v, w = map(int, input().split())
    u, v, w = u - 1, v - 1, w - 1
    adjs[u].append((v, w))
    adjs[v].append((u, w))

label = [-1 for _ in range(N)]
visit = [False for _ in range(N)]
stack = []

stack.append(0)
visit[0] = True
label[0] = 0

while len(stack) > 0:
    u = stack.pop()

    for (v, w) in adjs[u]:
        if not visit[v]:
            label[v] = w if label[u] != w else (w + 1) % N
            visit[v] = True
            stack.append(v)

print('\n'.join(map(str, [c + 1 for c in label])))
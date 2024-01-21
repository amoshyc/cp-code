from collections import deque


M = int(input())
G = [[] for _ in range(9)]
for _ in range(M):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    G[u].append(v)
    G[v].append(u)

start = [-1 for _ in range(9)]
for i, p in enumerate(input().split()):
    p = int(p) - 1
    start[p] = i
start = tuple(start)

visit = set([start])
queue = deque([(start, 0)])

ans = -1
while len(queue) > 0:
    state, dist = queue.popleft()

    if state == (0, 1, 2, 3, 4, 5, 6, 7, -1):
        ans = dist
        break

    u = state.index(-1)
    for v in G[u]:
        new_state = list(state)
        new_state[u], new_state[v] = new_state[v], new_state[u]
        new_state = tuple(new_state)
        if new_state not in visit:
            queue.append((new_state, dist + 1))
            visit.add(new_state)

print(ans)

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

visit = set([tuple(start)])
queue = deque([(start, 0)])

ans = -1
while len(queue) > 0:
    state, dist = queue.popleft()

    if state == [0, 1, 2, 3, 4, 5, 6, 7, -1]:
        ans = dist
        break

    pos = state.index(-1)
    for v in G[pos]:
        new_state = state.copy()
        new_state[pos], new_state[v] = new_state[v], new_state[pos]
        if tuple(new_state) not in visit:
            queue.append((new_state, dist + 1))
            visit.add(tuple(new_state))

print(ans)
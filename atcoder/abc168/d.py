from collections import defaultdict
from collections import deque

graph = defaultdict(list)

V, E = map(int, input().split())
for _ in range(E):
    u, v = map(int, input().split())
    graph[u].append(v)
    graph[v].append(u)

ans = defaultdict(lambda: -1)
que = deque([1])
ans[1] = 1
while len(que) > 0:
    u = que.popleft()
    for v in graph[u]:
        if ans[v] == -1:
            ans[v] = u
            que.append(v)

if len(ans) != V:
    print('No')
else:
    print('Yes')
    for v in range(2, V + 1):
        print(ans[v])
from collections import deque

N = int(input())
P = []
V = set()

for _ in range(N):
    x, y = map(int, input().split())
    P.append((x, y))

cnt = 0
for (x, y) in P:
    if (x, y) in V:
        continue
    cnt += 1

    que = deque([(x, y)])
    V.add((x, y))

    while len(que) > 0:
        x, y = que.popleft()
        for nx, ny in [
            (x - 1, y - 1),
            (x - 1, y),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]:
            if (nx, ny) in P and (nx, ny) not in V:
                V.add((nx, ny))
                que.append((nx, ny))

print(cnt)

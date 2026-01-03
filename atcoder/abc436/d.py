from collections import deque, defaultdict

H, W = map(int, input().split())
S = [input() for _ in range(H)]

pos = defaultdict(list)
for r in range(H):
    for c in range(W):
        if S[r][c].isalpha():
            pos[S[r][c]].append((r, c))

inf = 10**9
dis = [[inf for _ in range(W)] for _ in range(H)]
que = deque()

dis[0][0] = 0
que.append((0, 0))

while len(que) > 0:
    r, c = que.popleft()

    # walk
    for nr, nc in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]:
        if nr < 0 or nr >= H or nc < 0 or nc >= W:
            continue
        if S[nr][nc] != "#" and dis[nr][nc] == inf:
            dis[nr][nc] = dis[r][c] + 1
            que.append((nr, nc))

    # warp
    for nr, nc in pos[S[r][c]]:
        if dis[nr][nc] == inf:
            dis[nr][nc] = dis[r][c] + 1
            que.append((nr, nc))
    pos[S[r][c]].clear()  # !!!!!!!!!!

if dis[-1][-1] == inf:
    print(-1)
else:
    print(dis[-1][-1])

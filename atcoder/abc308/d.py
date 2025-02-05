from collections import deque


H, W = map(int, input().split())
S = [input() for _ in range(H)]
T = "snuke"

inf = 10**10
dis = [[inf for _ in range(W)] for _ in range(H)]
que = deque()

dis[0][0] = 0
que.append((0, 0))

while len(que) > 0:
    r, c = que.popleft()
    for dr, dc in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
        nr, nc = r + dr, c + dc
        nd = dis[r][c] + 1
        if 0 <= nr < H and 0 <= nc < W:
            if dis[nr][nc] == inf and S[nr][nc] == T[nd % 5]:
                dis[nr][nc] = nd
                que.append((nr, nc))

if dis[H - 1][W - 1] == inf:
    print("No")
else:
    print("Yes")

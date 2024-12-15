from collections import deque

H, W, D = map(int, input().split())
arr = [input() for _ in range(H)]

inf = 10**10
dis = [[inf for _ in range(W)] for _ in range(H)]
que = deque()

for r in range(H):
    for c in range(W):
        if arr[r][c] == "H":
            dis[r][c] = 0
            que.append((r, c))

while len(que) > 0:
    r, c = que.popleft()
    for dr, dc in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
        nr, nc = r + dr, c + dc
        if 0 <= nr < H and 0 <= nc < W:
            if dis[nr][nc] == inf and arr[nr][nc] == ".":
                dis[nr][nc] = dis[r][c] + 1
                que.append((nr, nc))

ans = 0
for r in range(H):
    for c in range(W):
        if dis[r][c] <= D:
            ans += 1
print(ans)

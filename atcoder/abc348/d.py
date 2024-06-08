from collections import deque

H, W = map(int, input().split())
A = [list(input()) for _ in range(H)]

med = [[0 for _ in range(W)] for _ in range(H)]
N = int(input())
for _ in range(N):
    r, c, e = map(int, input().split())
    r, c = r - 1, c - 1
    med[r][c] = e

s, t = (0, 0), (0, 0)
for r in range(H):
    for c in range(W):
        if A[r][c] == "S":
            s = (r, c)
            A[r][c] = "."
        if A[r][c] == "T":
            t = (r, c)
            A[r][c] = "."

rem = [[-1 for _ in range(W)] for _ in range(H)]
que = deque()

if med[s[0]][s[1]] > 0:
    rem[s[0]][s[1]] = med[s[0]][s[1]]
    que.append(s)

while len(que) > 0:
    r, c = que.popleft()
    if rem[r][c] == 0:
        continue
    for dr, dc in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
        nr, nc = r + dr, c + dc
        if 0 <= nr < H and 0 <= nc < W and A[nr][nc] == ".":
            new_rem = max(rem[r][c] - 1, med[nr][nc])
            if new_rem > rem[nr][nc]:
                rem[nr][nc] = new_rem
                que.append((nr, nc))

print("Yes" if rem[t[0]][t[1]] >= 0 else "No")

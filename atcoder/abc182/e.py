import sys

input = sys.stdin.readline

H, W, N, M = map(int, input().split())
EMPTY = 0
BULB = 1
BLOCK = 2

grid = [[EMPTY for _ in range(W)] for _ in range(H)]
for _ in range(N):
    r, c = map(int, input().split())
    grid[r - 1][c - 1] = BULB
for _ in range(M):
    r, c = map(int, input().split())
    grid[r - 1][c - 1] = BLOCK

visL = [[False for _ in range(W)] for _ in range(H)]
visR = [[False for _ in range(W)] for _ in range(H)]
visT = [[False for _ in range(W)] for _ in range(H)]
visD = [[False for _ in range(W)] for _ in range(H)]
for r in range(H):
    for c in range(W):
        if grid[r][c] == BULB:
            visL[r][c] = True
            visR[r][c] = True
            visT[r][c] = True
            visD[r][c] = True

for r in range(H):
    for c in range(1, W):
        if grid[r][c] == EMPTY:
            visL[r][c] = False if grid[r][c - 1] == BLOCK else visL[r][c - 1]
    for c in range(W - 2, -1, -1):
        if grid[r][c] == EMPTY:
            visR[r][c] = False if grid[r][c + 1] == BLOCK else visR[r][c + 1]
for c in range(W):
    for r in range(1, H):
        if grid[r][c] == EMPTY:
            visT[r][c] = False if grid[r - 1][c] == BLOCK else visT[r - 1][c]
    for r in range(H - 2, -1, -1):
        if grid[r][c] == EMPTY:
            visD[r][c] = False if grid[r + 1][c] == BLOCK else visD[r + 1][c]

ans = 0
for r in range(H):
    for c in range(W):
        if any([visL[r][c], visR[r][c], visT[r][c], visD[r][c]]):
            ans += 1
print(ans)
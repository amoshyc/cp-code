from collections import deque

def solve():
    H, W = map(int, input().split())
    A = [input() for _ in range(H)]

    sr, sc = -1, -1
    for r in range(H):
        for c in range(W):
            if A[r][c] == 'S':
                sr, sc = r, c

    vis = [[set() for _ in range(W)] for _ in range(H)]
    edges = [(+1, 0), (-1, 0), (0, +1), (0, -1)]

    for i, (dr, dc) in enumerate(edges):
        r, c = sr + dr, sc + dc
        if r < 0 or r >= H or c < 0 or c >= W or A[r][c] != '.':
            continue

        que = deque([(r, c)])
        while len(que) > 0:
            r, c = que.popleft()
            for (rr, cc) in edges:
                nr, nc = r + rr, c + cc
                if nr < 0 or nr >= H or nc < 0 or nc >= W or A[nr][nc] != '.':
                    continue
                if i not in vis[nr][nc]:
                    vis[nr][nc].add(i)
                    que.append((nr, nc))        

    for r in range(H):
        for c in range(W):
            if len(vis[r][c]) >= 2:
                return 'Yes'
    return 'No'


print(solve())
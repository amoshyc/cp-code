H, W = map(int, input().split())
A = [[int(x) for x in input().split()] for _ in range(H)]

cnt = 0
vis = set()

def dfs(r, c):
    if r == H - 1 and c == W - 1:
        global cnt
        cnt += 1
        return
    
    for dr, dc in [(0, +1), (+1, 0)]:
        nr, nc = r + dr, c + dc
        if nr < 0 or nr >= H or nc < 0 or nc >= W:
            continue
        if A[nr][nc] not in vis:
            vis.add(A[nr][nc])
            dfs(nr, nc)
            vis.remove(A[nr][nc])

vis.add(A[0][0])
dfs(0, 0)
print(cnt)
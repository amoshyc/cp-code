H, W = map(int, input().split())
S = [input() for _ in range(H)]

ans = set()
for r in range(H):
    for c in range(W):
        if S[r][c] == '.':
            for dr, dc in [(+1, 0), (-1, 0), (0, +1), (0, -1)]:
                nr = r + dr
                nc = c + dc
                if 0 <= nr < H and 0 <= nc < W and S[nr][nc] == '.':
                    ans.add((min(r, nr), min(c, nc), max(r, nr), max(c, nc)))
# print(ans)
print(len(ans))
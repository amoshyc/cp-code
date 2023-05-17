S = [input() for _ in range(9)]

ans = 0
for r1 in range(9):
    for c1 in range(9):
        for r2 in range(9):
            for c2 in range(9):
                dr = r2 - r1
                dc = c2 - c1
                A = (r1, c1)
                B = (r2, c2)
                C = (r1 - dc, c1 + dr)
                D = (r2 - dc, c2 + dr)
                pts = set([A, B, C, D])
                oks = [S[r][c] == '#' for (r, c) in pts if 0 <= r < 9 and 0 <= c < 9]
                if len(oks) == 4 and all(oks):
                    ans += 1

print(ans // 4)
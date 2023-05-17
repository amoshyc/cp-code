S = [input() for _ in range(10)]

r1, r2 = -1, -1
c1, c2 = -1, -1

for r in range(10):
    for c in range(10):
        if S[r][c] == '#':
            r2 = r
            c2 = c
            if r1 == -1:
                r1 = r
                c1 = c

print(r1 + 1, r2 + 1)
print(c1 + 1, c2 + 1)
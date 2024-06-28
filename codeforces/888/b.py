N = int(input())
S = list(input())

x, y = 0, 0
for c in S:
    if c == 'U':
        y += 1
    if c == 'D':
        y -= 1
    if c == 'L':
        x -= 1
    if c == 'R':
        x += 1

print(N - abs(x) - abs(y))
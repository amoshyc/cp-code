x1, y1 = map(int, input().split())
x2, y2 = map(int, input().split())
x3, y3 = map(int, input().split())

if x1 == x2:
    x4 = x3
if x2 == x3:
    x4 = x1
if x1 == x3:
    x4 = x2

if y1 == y2:
    y4 = y3
if y2 == y3:
    y4 = y1
if y3 == y1:
    y4 = y2

print(x4, y4)
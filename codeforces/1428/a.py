def solve():
    x1, y1, x2, y2 = map(int, input().split())

    if x1 == x2 and y1 == y2:
        return 0
    if x1 == x2:
        return abs(y2 - y1)
    if y1 == y2:
        return abs(x2 - x1)
    return abs(x2 - x1) + abs(y2 - y1) + 2

TC = int(input())
for _ in range(TC):
    print(solve())
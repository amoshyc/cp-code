a, b, x, y = map(int, input().split())

if a == b:
    print(x)
elif a > b: # b go up
    print(x + (a - 1 - b) * min(2 * x, y))
else: # b go down
    print(x + (b - a) * min(2 * x, y))

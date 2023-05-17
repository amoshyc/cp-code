def gcd(a, b):
    while b:
        a, b = b, a % b
    return a

def sign(x):
    return +1 if x > 0 else -1

N = int(input())
xy = []
for _ in range(N):
    x, y = map(int, input().split())
    xy.append((x, y))

need = set()
for i in range(N):
    for j in range(N):
        if i != j:
            xi, yi = xy[i]
            xj, yj = xy[j]
            dx, dy = xj - xi, yj - yi

            if dy == 0:
                need.add((sign(dx), 0))
            elif dx == 0:
                need.add((0, sign(dy)))
            else:
                g = gcd(abs(dx), abs(dy))
                dx //= g
                dy //= g
                need.add((dx, dy))
print(len(need))
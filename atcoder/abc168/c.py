from math import sin, cos, sqrt, pi

def polar2euclidean(r, theta):
    x = r * cos(theta)
    y = r * sin(theta)
    return x, y

A, B, H, M = map(int, input().split())
x1, y1 = polar2euclidean(A, -(H * 60 + M) / 720 * 2 * pi)
x2, y2 = polar2euclidean(B, -M / 60 * 2 * pi)
ans = sqrt((x2 - x1) ** 2 + (y2 - y1) ** 2)
print(ans)
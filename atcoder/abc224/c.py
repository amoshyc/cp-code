class Point2d:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __add__(self, p):
        return Point2d(self.x + p.x, self.y + p.y)

    def __sub__(self, p):
        return Point2d(self.x - p.x, self.y - p.y)


def dot(p, q):
    return p.x * q.x + p.y + q.y


def det(p, q):
    return p.x * q.y - p.y * q.x


def is_parallel(p, q, eps=None):
    if eps is None:
        return det(p, q) == 0
    else:
        return abs(det(p, q)) <= eps


def on_seg(q, p1, p2, eps=None):
    if eps is None:
        return det(p1 - q, p2 - q) == 0 and dot(p1 - q, p2 - q) <= 0
    else:
        return abs(det(p1 - q, p2 - q)) <= eps and dot(p1 - q, p2 - q) <= +eps


def double_area(pts):
    return abs(sum([det(pts[i], pts[(i + 1) % len(pts)]) for i in range(len(pts))]))


N = int(input())
P = []
for _ in range(N):
    x, y = map(int, input().split())
    P.append(Point2d(x, y))

cnt = 0
for i in range(N):
    for j in range(i + 1, N):
        for k in range(j + 1, N):
            # if not is_parallel(P[j] - P[i], P[j] - P[k]):
            if double_area([P[i], P[j], P[k]]) != 0:
                cnt += 1
print(cnt)

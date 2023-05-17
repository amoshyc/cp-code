class Point2d:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __add__(self, p):
        return Point2d(self.x + p.x, self.y + p.y)

    def __sub__(self, p):
        return Point2d(self.x - p.x, self.y - p.y)

    def __str__(self):
        return f'Point2d({self.x}, {self.y})'


def dot(p, q):
    return p.x * q.x + p.y * q.y

def det(p, q):
    return p.x * q.y - p.y * q.x


def solve():
    pts = [
        Point2d(*[int(x) for x in input().split()]),
        Point2d(*[int(x) for x in input().split()]),
        Point2d(*[int(x) for x in input().split()]),
        Point2d(*[int(x) for x in input().split()]),
    ]

    for i in range(4):
        v1 = pts[i] - pts[i - 1]
        v2 = pts[(i + 1) % 4] - pts[i]
        # print(v1, v2, det(v1, v2))
        if det(v1, v2) < 0:
            return False
    return True

print('Yes' if solve() else 'No')
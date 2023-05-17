from math import sqrt

class DisjoinSet:
    def __init__(self, N):
        self.par = [-1] * N
        self.sz = [1] * N

    def root(self, x):
        if self.par[x] < 0:
            return x
        else:
            self.par[x] = self.root(self.par[x])
            return self.par[x]

    def unite(self, a, b):
        a = self.root(a)
        b = self.root(b)
        if a == b:
            return
        if self.sz[a] > self.sz[b]:
            a, b = b, a
        self.par[a] = b
        self.sz[b] += self.sz[a]

    def same(self, a, b):
        return self.root(a) == self.root(b)

    def size(self, x):
        return self.sz[self.root(x)]

    def __str__(self):
        from collections import defaultdict
        clusters = defaultdict(list)
        for x in range(N):
            clusters[self.root(x)].append(x)
        return str(list(clusters.values()))


N = int(input())
xs, ys = [], []
for _ in range(N):
    x, y = map(float, input().split())
    xs.append(x)
    ys.append(y)


def check(r):
    dsu = DisjoinSet(N + 2)
    for i in range(N):
        for j in range(i + 1, N):
            if sqrt((xs[i] - xs[j]) ** 2 + (ys[i] - ys[j]) ** 2) < 2 * r:
                dsu.unite(i, j)
    for i in range(N):
        if abs(ys[i] - 100) < 2 * r:
            dsu.unite(i, N + 0)
        if abs(ys[i] - (-100)) < 2 * r:
            dsu.unite(i, N + 1)
    return dsu.root(N + 0) != dsu.root(N + 1)


lb, ub = 0, 200
for _ in range(50):
    m = (lb + ub) / 2
    if check(m):
        lb = m
    else:
        ub = m
print('{:.9f}'.format(lb))
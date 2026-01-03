from bisect import bisect_left
from collections import defaultdict


class DSU:
    def __init__(self, n):
        super().__init__()
        self.par = list(range(n))
        self.siz = [1 for _ in range(n)]

    def root(self, u):
        if self.par[u] == u:
            return u
        else:
            self.par[u] = self.root(self.par[u])
            return self.par[u]

    def unite(self, u, v):
        u, v = self.root(u), self.root(v)
        if u == v:
            return
        if self.siz[u] < self.siz[v]:
            self.par[u] = v
            self.siz[v] += self.siz[u]
        else:
            self.par[v] = u
            self.siz[u] += self.siz[v]

    def same(self, u, v):
        return self.root(u) == self.root(v)


N = int(input())
XR = [[int(x) for x in input().split()] for _ in range(N)]

xs = []
for x, r in XR:
    xs.append(x - r)
    xs.append(x + r)
xs = sorted(set(xs))

edges = []
for x, r in XR:
    u = bisect_left(xs, x - r)
    v = bisect_left(xs, x + r)
    edges.append((u, v))

V = len(xs)
dsu = DSU(V)
in_cycle = [False for _ in range(V)]
for u, v in edges:
    if dsu.same(u, v):
        in_cycle[u] = True
        in_cycle[v] = True
    else:
        dsu.unite(u, v)

ccs = defaultdict(list)
for u in range(V):
    ccs[dsu.root(u)].append(u)

ans = 0
for cc in ccs.values():
    if any(in_cycle[u] for u in cc):
        ans += len(cc)
    else:
        ans += len(cc) - 1

print(ans)

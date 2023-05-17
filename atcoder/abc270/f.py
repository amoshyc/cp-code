class DisjointSet:
    def __init__(self, N):
        self.N = N
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
            return None, None
        if self.sz[a] > self.sz[b]:
            a, b = b, a
        self.par[a] = b
        self.sz[b] += self.sz[a]
        return a, b  # tree a merges into tree b

    def same(self, a, b):
        return self.root(a) == self.root(b)

    def size(self, x):
        return self.sz[self.root(x)]


def kruskal(V, E):
    dsu = DisjointSet(V)
    cnt = 0
    for (u, v, w) in sorted(E, key=lambda e: e[2]):
        if not dsu.same(u, v):
            dsu.unite(u, v)
            cnt += w
    return cnt, dsu


INF = 10 ** 18
N, M = map(int, input().split())
X = [int(x) for x in input().split()]
Y = [int(x) for x in input().split()]
Er = []
for _ in range(M):
    u, v, z = map(int, input().split())
    u, v = u - 1, v - 1
    Er.append((u, v, z))
Ea = [(N + 0, i, x) for i, x in enumerate(X)]
Eh = [(N + 1, i, y) for i, y in enumerate(Y)]

cands = [
    kruskal(N + 2, Er),
    kruskal(N + 2, [*Er, *Ea]),
    kruskal(N + 2, [*Er, *Eh]),
    kruskal(N + 2, [*Er, *Ea, *Eh]),
]

ans = 10 ** 18
for cnt, dsu in cands:
    if dsu.size(0) >= N:
        ans = min(ans, cnt)
print(ans)
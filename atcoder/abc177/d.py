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
        clusters = defaultdict(list)
        for x in range(N):
            clusters[self.root(x)].append(x)
        return str(list(clusters.values()))


N, M = map(int, input().split())
dsu = DisjoinSet(N)
for _ in range(M):
    x, y = map(int, input().split())
    x, y = x - 1, y - 1
    dsu.unite(x, y)

ans = max([dsu.size(x) for x in range(N)])
print(ans)
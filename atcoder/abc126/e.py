class DisjointSet:
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

N, M = map(int, input().split())
disjoint = DisjointSet(N)
for _ in range(M):
    x, y, z = map(int, input().split())
    x, y = x - 1, y - 1
    disjoint.unite(x, y)

roots = set()
for i in range(N):
    roots.add(disjoint.root(i))
print(len(roots))
class DisjointSet:
    def __init__(self, N):
        self.N = N
        self.par = list(range(N))
        self.sz = [1] * N # only valid at roots

    def root(self, x):
        if self.par[x] == x:
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
dsu = DisjointSet(N)
cnt = 0
for _ in range(M):
    x, y = map(int, input().split())
    x, y = x - 1, y - 1
    if dsu.same(x, y):
        cnt += 1
    else:
        dsu.unite(x, y)
print(cnt)


class DisjointSet:
    def __init__(self, N):
        self.N = N
        self.par = list(range(N))
        self.siz = [1 for _ in range(N)]

    def root(self, x):
        if self.par[x] == x:
            return x
        else:
            self.par[x] = self.root(self.par[x])
            return self.par[x]

    def unite(self, x, y):
        x, y = self.root(x), self.root(y)
        if x == y:
            return
        if self.siz[x] > self.siz[y]:
            x, y = y, x
        self.par[x] = y
        self.siz[y] += self.siz[x]

    def same(self, x, y):
        return self.root(x) == self.root(y)

    def size(self, x):
        return self.siz[self.root(x)]


N = int(input())
A = [int(x) - 1 for x in input().split()]
M = max(A)

dsu = DisjointSet(M + 1)
for i in range(N):
    if A[i] != A[N - 1 - i]:
        dsu.unite(A[i], A[N - 1 - i])

ans = 0
for x in range(M + 1):
    if dsu.root(x) == x and dsu.size(x) >= 2:
        ans += dsu.size(x) - 1

print(ans)

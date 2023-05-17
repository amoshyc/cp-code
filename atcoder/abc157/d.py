from collections import defaultdict


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


N, M, K = map(int, input().split())

A, B = [], []
for _ in range(M):
    a, b = map(int, input().split())
    A.append(a - 1)
    B.append(b - 1)

C, D = [], []
for _ in range(K):
    c, d = map(int, input().split())
    C.append(c - 1)
    D.append(d - 1)

data = DisjoinSet(N)
for a, b in zip(A, B):
    data.unite(a, b)

ans = [data.size(x) - 1 for x in range(N)]

for a, b in zip(A, B):
    ans[a] -= 1
    ans[b] -= 1

for c, d in zip(C, D):
    if data.same(c, d):
        ans[c] -= 1
        ans[d] -= 1

print(' '.join(map(str, ans)))

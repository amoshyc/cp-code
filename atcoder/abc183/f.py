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
            return None, None
        if self.sz[a] > self.sz[b]:
            a, b = b, a
        self.par[a] = b
        self.sz[b] += self.sz[a]
        return a, b

    def same(self, a, b):
        return self.root(a) == self.root(b)

    def size(self, x):
        return self.sz[self.root(x)]

    def __str__(self):
        clusters = defaultdict(list)
        for x in range(N):
            clusters[self.root(x)].append(x)
        return str(list(clusters.values()))


N, Q = map(int, input().split())
C = [int(x) for x in input().split()]

dsu = DisjoinSet(N)
rec = [defaultdict(int, {C[i]: 1}) for i in range(N)]

for _ in range(Q):
    cmd, a, b = map(int, input().split())

    if cmd == 1:
        a, b = a - 1, b - 1
        src, dst = dsu.unite(a, b)
        if src is not None:
            for k, v in rec[src].items():
                rec[dst][k] += v
            rec[src].clear()
    else:
        a = a - 1
        root = dsu.root(a)
        print(rec[root][b])
        

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

N = int(input())
dsu = DisjointSet(N)
deg = [0 for _ in range(N)]
childs = []
for _ in range(N - 1):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    deg[u] += 1
    deg[v] += 1

    if u == 0:
        childs.append(v)
    elif v == 0:
        childs.append(u)
    else:
        dsu.unite(u, v)

if deg[0] == 1:
    print(1)
    exit()

sizes = [dsu.size(u) for u in childs]
print(sum(sizes) - max(sizes) + 1)
from collections import defaultdict


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


N, M = map(int, input().split())
adj = [[] for _ in range(N)]
dsu = DisjointSet(N)
for _ in range(M):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    adj[u].append(v)
    adj[v].append(u)
    dsu.unite(u, v)

v_cnt = defaultdict(int)
e_cnt = defaultdict(int)

for u in range(N):
    v_cnt[dsu.root(u)] += 1
    for v in adj[u]:
        e_cnt[dsu.root(u)] += 1

for k in v_cnt.keys():
    if v_cnt[k] * 2 != e_cnt[k]:
        print("No")
        exit()
print("Yes")

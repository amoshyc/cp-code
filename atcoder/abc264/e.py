from collections import defaultdict


class DisjointSet:
    def __init__(self, N):
        self.par = [-1] * N
        self.sz = [1] * N
        self.on = [False] * N

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
        self.on[b] |= self.on[a]
        self.sz[b] += self.sz[a]
        return a, b  # tree a merges into tree b

    def same(self, a, b):
        return self.root(a) == self.root(b)

    def size(self, x):
        return self.sz[self.root(x)]

    def __str__(self):
        clusters = defaultdict(list)
        for x in range(len(self.par)):
            clusters[self.root(x)].append(x + 1)
        return str(list(clusters.values()))


N, M, E = map(int, input().split())
edges = []
for _ in range(E):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    edges.append((u, v))
Q = int(input())
events = [int(input()) - 1 for _ in range(Q)]

check = set(events)

T = DisjointSet(N + M)
for i in range(M):
    T.on[N + i] = True

for i, (u, v) in enumerate(edges):
    if i not in check:
        T.unite(u, v)
cnt = sum([1 for u in range(N) if T.on[T.root(u)]])

# print(T)
# print(cnt)

ans = [-1 for _ in range(Q)]
ans[-1] = cnt

for i in range(Q - 2, -1, -1):
    u, v = edges[events[i + 1]]
    u, v = T.root(u), T.root(v)

    if (not T.on[u]) and T.on[v]:
        cnt += T.size(u)
    if T.on[u] and (not T.on[v]):
        cnt += T.size(v)
    ans[i] = cnt
    T.unite(u, v)

print('\n'.join(map(str, ans)))


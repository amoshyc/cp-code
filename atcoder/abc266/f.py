from collections import defaultdict, deque


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

    def __str__(self):
        clusters = defaultdict(list)
        for x in range(self.N):
            clusters[self.root(x)].append(x)
        return str(list(clusters.values()))


N = int(input())

tree = DisjointSet(N)
adj = [[] for _ in range(N)]
deg = [0 for _ in range(N)]

for _ in range(N):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    adj[u].append(v)
    adj[v].append(u)
    deg[u] += 1
    deg[v] += 1

que = deque([u for u in range(N) if deg[u] == 1])
while len(que) > 0:
    u = que.popleft()
    for v in adj[u]:
        tree.unite(u, v)
        deg[v] -= 1
        if deg[v] == 1:
            que.append(v)

# print(tree)

Q = int(input())
for _ in range(Q):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    print('Yes' if tree.same(u, v) else 'No')


from collections import deque


def bfs01(adj, src, inf):
    n = len(adj)
    dis = [inf for _ in range(n)]
    que = deque()

    dis[src] = 0
    que.append((src, dis[src]))

    while que:
        u, d = que.popleft()
        if d > dis[u]:
            continue
        for v, w in adj[u]:
            nd = d + w
            if dis[v] > nd:
                dis[v] = nd
                if w == 0:
                    que.appendleft((v, nd))
                else:
                    que.append((v, nd))

    return dis


TC = int(input())

#  0
# 1 3
#  2
dirs = "ABC"
valid = [
    [(1, 3), (0, 2), (2, 0), (3, 1)],  # A
    [(1, 2), (0, 3), (2, 1), (3, 0)],  # B
    [(1, 0), (0, 1), (2, 3), (3, 2)],  # C
]

ans = []
for _ in range(TC):
    H, W = map(int, input().split())
    S = [input() for _ in range(H)]

    def id(r, c, d):
        return (r * W + c) * 4 + d

    adj = [[] for _ in range(4 * H * W)]

    # Identity edges
    for r in range(H):
        for c in range(W):
            if r >= 1:
                adj[id(r, c, 0)].append((id(r - 1, c, 2), 0))
            if c >= 1:
                adj[id(r, c, 1)].append((id(r, c - 1, 3), 0))
            if r + 1 < H:
                adj[id(r, c, 2)].append((id(r + 1, c, 0), 0))
            if c + 1 < W:
                adj[id(r, c, 3)].append((id(r, c + 1, 1), 0))

    # Change (r, c) to k
    for r in range(H):
        for c in range(W):
            for k in range(3):
                cost = 0 if S[r][c] == dirs[k] else 1
                for src, dst in valid[k]:
                    adj[id(r, c, src)].append((id(r, c, dst), cost))

    dis = bfs01(adj, id(0, 0, 1), 10**8)
    ans.append(dis[id(H - 1, W - 1, 3)])

print("\n".join(map(str, ans)))

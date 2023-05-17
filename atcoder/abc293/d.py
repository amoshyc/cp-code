N, M = map(int, input().split())

adj = [[] for _ in range(2 * N)]
for i in range(N):
    adj[2 * i + 0].append(2 * i + 1)
    adj[2 * i + 1].append(2 * i + 0)

for _ in range(M):
    a, b, c, d = input().split()
    u = (int(a) - 1) * 2 + (1 if b == 'B' else 0)
    v = (int(c) - 1) * 2 + (1 if d == 'B' else 0)
    adj[u].append(v)
    adj[v].append(u)

cnt1, cnt2 = 0, 0
vis = [False for _ in range(2 * N)]
for root in range(2 * N):
    if vis[root]:
        continue

    stack = [root]
    verts = set([root])
    while len(stack) > 0:
        u = stack.pop()
        for v in adj[u]:
            if not vis[v]:
                vis[v] = True
                stack.append(v)
                verts.add(v)

    # print(root, verts, [len(adj[v]) for v in verts])
    if all(len(adj[v]) % 2 == 0 for v in verts):
        cnt1 += 1
    else:
        cnt2 += 1

print(cnt1, cnt2)
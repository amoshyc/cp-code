N, K = map(int, input().split())
P = list(map(int, input().split()))
P = [p - 1 for p in P]
C = list(map(int, input().split()))

vis = [False] * N
ans = [-float('inf')] * N

for root in range(N):
    if vis[root]:
        continue
    vis[root] = True

    cycle = [root]
    costs = [C[root]]
    u = P[root]
    vis[u] = True
    while u != root:
        cycle.append(u)
        costs.append(C[u])
        u = P[u]
        vis[u] = True

    # print('-' * 10)
    # print(cycle)
    # print(costs)

    L = len(cycle)
    S = sum(costs)
    for i, v in enumerate(cycle):
        cnt = 0
        for step in range(1, min(L, K) + 1):
            cnt += costs[(i + step) % L]
            val1 = cnt
            val2 = cnt + (K - step) // L * S
            ans[v] = max(ans[v], val1, val2)

print(max(ans))


N, K = map(int, input().split())
P = list(map(int, input().split()))
P = [p - 1 for p in P]
C = list(map(int, input().split()))

vis = [False] * N
ans = [-float('inf')] * N

for root in range(N):
    if vis[root]:
        continue

    cycle = [root]
    costs = [C[root]]
    u = P[root]
    while u != root:
        cycle.append(u)
        costs.append(C[u])
        u = P[u]
    
    for u in cycle:
        vis[u] = True

    L = len(cycle)
    S = sum(costs)
    for i, v in enumerate(cycle):
        # Rewrite [1, K] as form q * L + r,
        # 0L + 1, 0L + 2, ..., 0L + L
        # 1L + 1, 1L + 2, ..., 1L + L
        # ⋮ 
        # tL + 1, tL + 2, ... 

        # We can ignore 1L + r, 2L + r, ..., (t - 1) * L + r
        # since the total cost of loop is either > 0 or <= 0
        # When > 0, 0L + r < 1L + r < 2L + r < tL + r
        # When <= 0, the answer should be 0L + r

        if S > 0:
            rem_cost = 0
            for rem in range(1, min(L, K) + 1):
                rem_cost += costs[(i + rem) % L]
                t = (K - rem) // L
                ans[v] = max(ans[v], rem_cost + t * S)
        else:
            rem_cost = 0
            for rem in range(1, min(L, K) + 1):
                rem_cost += costs[(i + rem) % L]
                ans[v] = max(ans[v], rem_cost)

print(max(ans))


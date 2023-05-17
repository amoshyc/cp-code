# 5 5 0
# 1 1 1 1 1
# 1 2
# 2 3
# 3 4
# 4 5
# expects 2 ** 4 = 16

N, M, K = map(int, input().split())
A = [int(x) - 1 for x in input().split()]
G = [[] for _ in range(N)]
C = dict()
for _ in range(N - 1):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    G[u].append(v)
    G[v].append(u)
    C[(min(u, v), max(u, v))] = 0

for i in range(1, M):
    s, t = A[i - 1], A[i]
    stack = [s]
    visit = [False for _ in range(N)]
    parent = [-1 for _ in range(N)]
    visit[s] = True
    parent[s] = -1
    while len(stack) > 0:
        u = stack.pop()
        if u == t:
            break
        for v in G[u]:
            if not visit[v]:
                stack.append(v)
                parent[v] = u
                visit[v] = True
    path = [t]
    while parent[path[-1]] != -1:
        path.append(parent[path[-1]])
    path.reverse()
    for u, v in zip(path[:-1], path[1:]):
        C[(min(u, v), max(u, v))] += 1
C = list(C.values())
S = sum(C)

# dp[i][j] = number of ways to form j from C[0], C[1], ..., C[i]
# dp[i][j] = dp[i - 1][j - C[i]]
V = 10 ** 5
dp = [0 for _ in range(V + 1)]
dp[0] = 1
mod = 998244353
for i in range(len(C)):
    for j in range(V, C[i] - 1, -1):
        dp[j] += dp[j - C[i]]
        dp[j] %= mod
if (K + S) % 2 == 1 or (K + S) < 0:
    print(0)
else:
    print(dp[(K + S) // 2])

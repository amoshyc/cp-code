N, M = map(int, input().split())
P = [int(x) - 1 for x in input().split()]

childs = [[] for _ in range(N)]
for i, p in enumerate(P):
    childs[p].append(i + 1) # aware the i + 1

dp = [0 for _ in range(N)]
for _ in range(M):
    x, y = map(int, input().split())
    x = x - 1
    dp[x] = max(dp[x], y + 1) # y + 1, not y

stk = [0]
while len(stk) > 0:
    u = stk.pop()
    for v in childs[u]:
        stk.append(v)
        if dp[u] >= 1:
            dp[v] = max(dp[v], dp[u] - 1)
        
print(len([x for x in dp if x > 0]))
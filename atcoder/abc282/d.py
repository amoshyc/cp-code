N, M = map(int, input().split())

adj = [[] for _ in range(N)]
for _ in range(M):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    adj[u].append(v)
    adj[v].append(u)

color = [-1 for _ in range(N)]
cnts = []
for r in range(N):
    if color[r] == -1:
        color[r] = 0
        cnt0, cnt1 = 0, 0
        stack = [r]
        while len(stack) > 0:
            u = stack.pop()
            if color[u] == 0:
                cnt0 += 1
            else:
                cnt1 += 1
            for v in adj[u]:
                if color[v] == -1:
                    color[v] = color[u] ^ 1
                    stack.append(v)
        
        cnts.append((cnt0, cnt1))

for u in range(N):
    for v in adj[u]:
        if color[u] == color[v]:
            print(0)
            exit()

ans1 = 0
for (cnt0, cnt1) in cnts:
    ans1 = ans1 + cnt0 * cnt1
ans2 = 0
for (cnt0, cnt1) in cnts:
    ans2 = ans2 + (cnt0 + cnt1) * (N - cnt0 - cnt1)
ans2 //= 2

print(ans1 + ans2 - M)
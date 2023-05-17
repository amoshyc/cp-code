from collections import defaultdict

def solve():
    N = int(input())
    adj = defaultdict(list)
    for _ in range(N):
        a, b = map(int, input().split())
        adj[a].append(b)
        adj[b].append(a)

    vis = defaultdict(bool)
    vis[1] = True
    stack = [1]

    ans = 1
    while len(stack) > 0:
        u = stack.pop()
        ans = max(ans, u)
        for v in adj[u]:
            if not vis[v]:
                vis[v] = True
                stack.append(v)
    
    return ans

print(solve())
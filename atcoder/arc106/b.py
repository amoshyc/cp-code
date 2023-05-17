N, M = map(int, input().split())
A = [int(x) for x in input().split()]
B = [int(x) for x in input().split()]

adj = [[] for _ in range(N)]
for _ in range(M):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    adj[u].append(v)
    adj[v].append(u)


def solve():
    vis = [False] * N
    for root in range(N):
        if vis[root]:
            continue

        vis[root] = True
        src, tgt = 0, 0
        stack = [root]
        while len(stack) > 0:
            u = stack.pop()
            src += A[u]
            tgt += B[u]
            for v in adj[u]:
                if not vis[v]:
                    vis[v] = True
                    stack.append(v)

        if src != tgt:
            return 'No'

    return 'Yes'


print(solve())

from collections import deque

N = int(input())
X = [0 for _ in range(N)]
Y = [0 for _ in range(N)]
P = [0 for _ in range(N)]
for i in range(N):
    X[i], Y[i], P[i] = list(map(int, input().split()))


def ok(s: int) -> bool:
    for root in range(N):
        que = deque()
        vis = [False for _ in range(N)]

        que.append(root)
        vis[root] = True

        while len(que) > 0:
            u = que.popleft()
            for v in range(N):
                if not vis[v] and s * P[u] >= abs(X[u] - X[v]) + abs(Y[u] - Y[v]):
                    vis[v] = True
                    que.append(v)

        if all(vis):
            return True

    return False


# 0 0 0 1 1 1
lb, ub = 0, 10**10
while ub - lb > 1:
    m = (lb + ub) // 2
    if ok(m):
        ub = m
    else:
        lb = m

print(ub)

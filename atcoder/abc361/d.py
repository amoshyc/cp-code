from collections import defaultdict, deque

N = int(input())
S = input() + ".."
T = input() + ".."

inf = 10**10
dis = defaultdict(lambda: inf)
que = deque()

dis[S] = 0
que.append(S)

while len(que) > 0:

    u = que.popleft()
    if u == T:  # early stopping
        print(dis[u])
        exit()

    # p is the position of the first '.'
    p = next(i for i, c in enumerate(u) if c == ".")

    # Inspect the neighbors of u
    for i in range(len(u) - 1):
        if u[i] != "." and u[i + 1] != ".":
            # Single operation
            v = list(u)
            v[i], v[p] = v[p], v[i]
            v[i + 1], v[p + 1] = v[p + 1], v[i + 1]
            v = "".join(v)

            if dis[v] == inf:
                dis[v] = dis[u] + 1
                que.append(v)

print(-1)

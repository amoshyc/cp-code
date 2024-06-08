from collections import deque


N = int(input())
arr = [input() for _ in range(N)]
base = [1, N, N**2, N**3]


def encode(v):
    return sum(x * y for x, y in zip(v, base))


def decode(v):
    state = []
    for _ in range(4):
        state.append(v % N)
        v //= N
    return state


rs = [0, 0]
cs = [0, 0]
i = 0
for r in range(N):
    for c in range(N):
        if arr[r][c] == "P":
            rs[i] = r
            cs[i] = c
            i += 1

que = deque()
dis = dict()

root = encode([rs[0], cs[0], rs[1], cs[1]])
que.append(root)
dis[root] = 0

while len(que) > 0:
    k = que.popleft()
    d = dis[k]
    (r0, c0, r1, c1) = decode(k)

    if (r0, c0) == (r1, c1):
        print(d)
        exit()

    for dr, dc in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
        rr0, cc0 = r0, c0
        rr1, cc1 = r1, c1

        nr, nc = r0 + dr, c0 + dc
        if 0 <= nr < N and 0 <= nc < N and arr[nr][nc] != "#":
            rr0, cc0 = nr, nc

        nr, nc = r1 + dr, c1 + dc
        if 0 <= nr < N and 0 <= nc < N and arr[nr][nc] != "#":
            rr1, cc1 = nr, nc

        key = encode([rr0, cc0, rr1, cc1])
        if key not in dis:
            dis[key] = d + 1
            que.append(key)

print(-1)

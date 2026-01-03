N, Q = map(int, input().split())
A = [int(x) for x in input().split()]

A.insert(0, 0)
N = N + 1

nxt = [[0 for _ in range(N)] for _ in range(30)]
nxt[0] = A
for i in range(1, 30):
    for u in range(N):
        nxt[i][u] = nxt[i - 1][nxt[i - 1][u]]

cnt = [[0 for _ in range(N)] for _ in range(30)]
cnt[0] = list(range(N))
for i in range(1, 30):
    for u in range(N):
        cnt[i][u] = cnt[i - 1][u] + cnt[i - 1][nxt[i - 1][u]]

res = []
for _ in range(Q):
    t, b = map(int, input().split())
    v = 0
    for i in range(30):
        if (t >> i) & 1:
            v += cnt[i][b]
            b = nxt[i][b]
    res.append(v)

print("\n".join(map(str, res)))

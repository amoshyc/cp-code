class SegTree:
    def __init__(self, A, default, op):
        self.default = default
        self.op = op

        nn = 1
        while nn < len(A):
            nn *= 2
        self.nn = nn

        self.data = [default for _ in range(2 * nn)]
        self.data[nn - 1 : nn - 1 + len(A)] = A
        for u in range(nn - 2, -1, -1):
            self.data[u] = op(self.data[2 * u + 1], self.data[2 * u + 2])

    # query = [a, b), current at node u with [l, r)
    def query(self, a: int, b: int, u: int, l: int, r: int) -> int:
        # [l, r) not in [a, b)
        if r <= a or l >= b:
            return self.default
        # [l, r) is inside [a, b)
        if a <= l and r <= b:
            return self.data[u]
        # partially overlapped
        m = (l + r) // 2
        res1 = self.query(a, b, 2 * u + 1, l, m)
        res2 = self.query(a, b, 2 * u + 2, m, r)
        return self.op(res1, res2)

    # modify = [p, p + 1), current at node u with [l, r)
    def modify(self, p: int, x: int, u: int, l: int, r: int):
        # [l, r) not in [a, b)
        if r <= p or l >= p + 1:
            return
        # [l, r) is inside [a, b)
        if p <= l and r <= p + 1:
            self.data[u] += x
            return
        # partially overlapped
        m = (l + r) // 2
        self.modify(p, x, 2 * u + 1, l, m)
        self.modify(p, x, 2 * u + 2, m, r)
        self.data[u] = self.op(self.data[2 * u + 1], self.data[2 * u + 2])


H, W, M = map(int, input().split())
cntR = [0 for _ in range(H)]
cntC = [0 for _ in range(W)]
columns_in_row = [[] for _ in range(H)]
for _ in range(M):
    r, c = map(int, input().split())
    r, c = r - 1, c - 1
    cntR[r] += 1
    cntC[c] += 1
    columns_in_row[r].append(c)

# max_(0 <= r < H) max_(0 <= c < W) (cntR[r] + cntC[c] - A[r][c])
# = max_(0 <= r < H) (cntR[r] + max_(0 <= c < W) (cntC[c] - A[r][c]))

seg = SegTree(cntC, 0, max)
ans = 0
for r in range(H):
    # Subtract the duplicate count (A[r][c])
    for c in columns_in_row[r]:
        seg.modify(c, -1, 0, 0, seg.nn)

    val = cntR[r] + seg.query(0, W, 0, 0, seg.nn)
    ans = max(ans, val)

    # Restore the cntC
    for c in columns_in_row[r]:
        seg.modify(c, +1, 0, 0, seg.nn)

print(ans)

from operator import add


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
    def modify(self, p: int, x: int, u, l, r):
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


N, Q = map(int, input().split())
A = [int(x) for x in input().split()]

seg = SegTree(A, default=0, op=add)

ans = []

for _ in range(Q):
    cmd = [int(x) for x in input().split()]

    if cmd[0] == 0:
        p, x = cmd[1], cmd[2]
        seg.modify(p, x, 0, 0, seg.nn)
    else:
        l, r = cmd[1], cmd[2]
        ans.append(seg.query(l, r, 0, 0, seg.nn))

print("\n".join(map(str, ans)))

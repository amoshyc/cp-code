class LazySegTree:
    def default_val(self):
        return 10**18

    def default_lazy(self):
        return 0

    def op(self, a, b):
        return min(a, b)

    def apply_lazy(self, lazy, val, u, l, r):
        return lazy + val

    def merge_lazy(self, parent, child):
        return parent + child

    def __init__(self, A):
        self.NN = 1
        while self.NN < len(A):
            self.NN <<= 1
        self.data = [self.default_val() for _ in range(2 * self.NN)]
        self.lazy = [self.default_lazy() for _ in range(2 * self.NN)]
        self.data[self.NN - 1 : self.NN - 1 + len(A)] = A
        for u in range(self.NN - 2, -1, -1):
            self.data[u] = self.op(self.data[u * 2 + 1], self.data[u * 2 + 2])

    def push(self, u, l, r):
        if self.lazy[u] != self.default_lazy():
            m, lch, rch = (l + r) // 2, u * 2 + 1, u * 2 + 2
            self.data[lch] = self.apply_lazy(self.lazy[u], self.data[lch], lch, l, m)
            self.data[rch] = self.apply_lazy(self.lazy[u], self.data[rch], rch, m, r)
            self.lazy[lch] = self.merge_lazy(self.lazy[u], self.lazy[lch])
            self.lazy[rch] = self.merge_lazy(self.lazy[u], self.lazy[rch])
            self.lazy[u] = self.default_lazy()

    def prod(self, a, b):
        stack = [(0, 0, self.NN)]
        result = []
        while len(stack) > 0:
            u, l, r = stack.pop()
            if l >= b or r <= a:
                result.append(self.default_val())
                continue
            if l >= a and r <= b:
                result.append(self.data[u])
                continue
            m, lch, rch = (l + r) // 2, u * 2 + 1, u * 2 + 2
            self.push(u, l, r)
            stack.append((lch, l, m))
            stack.append((rch, m, r))
        while len(result) > 1:
            result.append(self.op(result.pop(), result.pop()))
        return result[0]

    def apply(self, a, b, lazy):
        stack = [(0, 0, self.NN)]
        verts = []
        while len(stack) > 0:
            u, l, r = stack.pop()
            if l >= b or r <= a:  # no intersection
                continue
            if l >= a and r <= b:  # [l, r) is inside [a, b)
                self.data[u] = self.apply_lazy(lazy, self.data[u], u, l, r)
                self.lazy[u] = self.merge_lazy(lazy, self.lazy[u])
                continue
            m, lch, rch = (l + r) // 2, u * 2 + 1, u * 2 + 2
            self.push(u, l, r)
            stack.append((lch, l, m))
            stack.append((rch, m, r))
            verts.append(u)
        for u in reversed(verts):
            self.data[u] = self.op(self.data[u * 2 + 1], self.data[u * 2 + 2])


N, Q = map(int, input().split())
S = list(input())

P = [0 for _ in range(N)]
P[0] = +1 if S[0] == "(" else -1
for i in range(1, N):
    P[i] = P[i - 1] + (+1 if S[i] == "(" else -1)

seg = LazySegTree(P)

for _ in range(Q):
    cmd, l, r = map(int, input().split())
    l, r = l - 1, r - 1

    if cmd == 1:
        if S[l] == "(" and S[r] == ")":
            seg.apply(l, r, -2)
            S[l], S[r] = S[r], S[l]
        elif S[l] == ")" and S[r] == "(":
            seg.apply(l, r, +2)
            S[l], S[r] = S[r], S[l]
    else:
        check1 = S[l] == "(" and S[r] == ")"
        check2 = seg.prod(r, r + 1) == (seg.prod(l - 1, l) if l > 0 else 0)
        check3 = seg.prod(l, r + 1) == seg.prod(l, l + 1) - 1

        if check1 and check2 and check3:
            print("Yes")
        else:
            print("No")

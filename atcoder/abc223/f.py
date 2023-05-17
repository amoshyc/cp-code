class LazySegTree:
    def default_val(self):
        return 0x3f3f3f3f

    def default_lazy(self):
        return 0

    def op(self, a, b):
        return min(a, b)

    def apply_lazy(self, lazy, val):
        return lazy + val

    def push_lazy(self, parent, child):
        return parent + child

    def __init__(self, N):
        self.H = 0
        self.size = 1
        while self.size < N:
            self.H += 1
            self.size <<= 1
        self.N = N
        self.data = [self.default_val() for _ in range(2 * self.size)]
        self.lazy = [self.default_lazy() for _ in range(self.size)]

    def update(self, k):
        self.data[k] = self.op(self.data[2 * k + 0], self.data[2 * k + 1])

    def all_apply(self, k, lazy):
        self.data[k] = self.apply_lazy(lazy, self.data[k])
        if k < self.size:
            self.lazy[k] = self.push_lazy(lazy, self.lazy[k])

    def push(self, k):
        self.all_apply(2 * k + 0, self.lazy[k])
        self.all_apply(2 * k + 1, self.lazy[k])
        self.lazy[k] = self.default_lazy()

    def set(self, pos, val):
        assert 0 <= pos < self.N
        pos += self.size
        for i in range(self.H, 0, -1):
            self.push(pos >> i)
        self.data[pos] = val
        for i in range(1, self.H + 1):
            self.update(pos >> i)

    def get(self, pos):
        assert 0 <= pos < self.N
        pos += self.size
        for i in range(self.H, 0, -1):
            self.push(pos >> i)
        return self.data[pos]

    def prod(self, l, r):
        assert 0 <= l and l <= r <= self.N
        if l == r:
            return self.default_val()

        l += self.size
        r += self.size

        for i in range(self.H, 0, -1):
            if ((l >> i) << i) != l:
                self.push(l >> i)
            if ((r >> i) << i) != r:
                self.push((r - 1) >> i)

        resL, resR = self.default_val(), self.default_val()
        while l < r:
            if l & 1:
                resL = self.op(resL, self.data[l])
                l += 1
            if r & 1:
                r -= 1
                resR = self.op(resR, self.data[r])
            l >>= 1
            r >>= 1

        return self.op(resL, resR)

    def apply(self, l, r, lazy):
        assert 0 <= l and l <= r <= self.N
        if l == r:
            return
        l += self.size
        r += self.size
        for i in range(self.H, 0, -1):
            if ((l >> i) << i) != l:
                self.push(l >> i)
            if ((r >> i) << i) != r:
                self.push((r - 1) >> i)

        L, R = l, r
        while L < R:
            if L & 1:
                self.all_apply(L, lazy)
                L += 1
            if R & 1:
                R -= 1
                self.all_apply(R, lazy)
            L >>= 1
            R >>= 1

        for i in range(1, self.H + 1):
            if ((l >> i) << i) != l:
                self.update(l >> i)
            if ((r >> i) << i) != r:
                self.update((r - 1) >> i)


N, Q = map(int, input().split())
S = list(input())

P = [0 for _ in range(N)]
P[0] = +1 if S[0] == '(' else -1
for i in range(1, N):
    P[i] = P[i - 1] + (+1 if S[i] == '(' else -1)

seg = LazySegTree(N)
for i in range(N):
    seg.set(i, P[i])

for _ in range(Q):
    cmd, l, r = map(int, input().split())
    l, r = l - 1, r - 1

    if cmd == 1:
        if S[l] == '(' and S[r] == ')':
            seg.apply(l, r, -2)
            S[l], S[r] = S[r], S[l]
        elif S[l] == ')' and S[r] == '(':
            seg.apply(l, r, +2)
            S[l], S[r] = S[r], S[l]
    else:
        check1 = S[l] == '(' and S[r] == ')'
        check2 = seg.get(r) - (seg.get(l - 1) if l > 0 else 0) == 0
        check3 = seg.prod(l, r + 1) == seg.get(l) - 1

        if check1 and check2 and check3:
            print('Yes')
        else:
            print('No')
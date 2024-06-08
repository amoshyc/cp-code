MOD = 998_244_353


class LazySegTree:
    def default_data(self):
        return 0

    def default_lazy(self):
        return (1, 0)

    def op(self, a, b):
        return (a + b) % MOD

    def apply_lazy(self, lazy, data, l, r):
        # b * data + c * (r - l)
        b, c = lazy
        res = b * data % MOD
        res += c * (r - l) % MOD
        res %= MOD
        return res

    def merge_lazy(self, parent, child):
        # b1 (b2 x + c2) + c1 = (b1 b2) x + (b1 c2 + c1)
        b1, c1 = parent
        b2, c2 = child
        b = b1 * b2 % MOD
        c = (b1 * c2 % MOD + c1) % MOD
        return (b, c)

    def __init__(self, arr):
        """
        Build a segment tree from arr
        """
        # Find the next power of 2 of len(arr)
        self.nn = 1
        while self.nn < len(arr):
            self.nn *= 2

        # Init data and lazy of each node
        self.data = [self.default_data() for _ in range(2 * self.nn)]
        self.lazy = [self.default_lazy() for _ in range(2 * self.nn)]

        # Copy arr to leaves
        self.data[self.nn - 1 : self.nn - 1 + len(arr)] = arr

        # Compute the data of internal nodes
        # In 0-based indexing, left child is 2u + 1, right child is 2u + 2
        # In 1-based indexing, left child is 2u + 0, right child is 2u + 1
        for u in range(self.nn - 2, -1, -1):
            self.data[u] = self.op(self.data[2 * u + 1], self.data[2 * u + 2])

    def push(self, u, l, r):
        """
        Pushing at node u, whose corresponding interval is [l, r)
        """
        # Do nothing if the lazy is not set
        if self.lazy[u] == self.default_lazy():
            return
        # Otherwise push it downward
        # 1. Update the data of its lch and rch
        # 2. Update the lazy of its lch and rch
        # 3. Reset the lazy of current node
        m, lch, rch = (l + r) // 2, 2 * u + 1, 2 * u + 2
        self.data[lch] = self.apply_lazy(self.lazy[u], self.data[lch], l, m)
        self.data[rch] = self.apply_lazy(self.lazy[u], self.data[rch], m, r)
        self.lazy[lch] = self.merge_lazy(self.lazy[u], self.lazy[lch])
        self.lazy[rch] = self.merge_lazy(self.lazy[u], self.lazy[rch])
        self.lazy[u] = self.default_lazy()

    def query(self, a, b, u, l, r):
        """
        Return the result of interval [a, b).
        Currently we are at node u, whose corresponding interval is [l, r).
        The user calls the query in the form seg.query(a, b, 0, 0, seg.nn)
        """
        # Case 1: [l, r) doesn't intersect with [a, b)
        # node u is not part of the answer.
        # Return a value that won't affect answer.
        if l >= b or r <= a:
            return self.default_data()
        # Case 2: [l, r) is inside [a, b]
        # data at node u is part of the answer.
        if l >= a and r <= b:
            return self.data[u]
        # Case 3: [l, r) partially intersect with [a, b)
        # Recursively go down to the left child and right child
        # Remember to push before going down.
        m, lch, rch = (l + r) // 2, 2 * u + 1, 2 * u + 2
        self.push(u, l, r)
        result1 = self.query(a, b, lch, l, m)
        result2 = self.query(a, b, rch, m, r)
        return self.op(result1, result2)

    def modify(self, a, b, x, u, l, r):
        """
        Return the interval [a, b) using lazy x.
        Currently we are at node u, whose corresponding interval is [l, r).
        The user calls the modify in the form seg.modify(a, b, x, 0, 0, seg.nn)
        """
        # Case 1: [l, r) doesn't intersect with [a, b)
        # Do nothing
        if l >= b or r <= a:
            return
        # Case 2: [l, r) is inside [a, b]
        # Modify the data of node u and set its lazy.
        if l >= a and r <= b:
            self.data[u] = self.apply_lazy(x, self.data[u], l, r)
            self.lazy[u] = self.merge_lazy(x, self.lazy[u])
            return
        # Case 3: [l, r) partially intersect with [a, b)
        # Recursively go down to the left child and right child
        # Remember to push before going down.
        # Since the data of left child and right child may be modified,
        # Recompute the data of this node aftwards.
        m, lch, rch = (l + r) // 2, 2 * u + 1, 2 * u + 2
        self.push(u, l, r)
        self.modify(a, b, x, lch, l, m)
        self.modify(a, b, x, rch, m, r)
        self.data[u] = self.op(self.data[lch], self.data[rch])


N, Q = map(int, input().split())
A = [int(x) for x in input().split()]

seg = LazySegTree(A)

ans = []
for _ in range(Q):
    cmd = [int(x) for x in input().split()]
    if cmd[0] == 0:
        l, r, b, c = cmd[1:]
        seg.modify(l, r, (b, c), 0, 0, seg.nn)
    else:
        l, r = cmd[1:]
        ans.append(seg.query(l, r, 0, 0, seg.nn))

print("\n".join(map(str, ans)))

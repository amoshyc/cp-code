from collections import defaultdict


class PolyHash:
    def __init__(self, N, base=31, mod=10**9 + 7):
        self.base = base
        self.mod = mod  # should be prime
        self.powr = [1 for _ in range(N)]
        self.pinv = [1 for _ in range(N)]
        for i in range(1, N):
            self.powr[i] = self.powr[i - 1] * base % mod
        self.pinv[-1] = pow(self.powr[-1], mod - 2, mod)
        for i in range(N - 2, -1, -1):
            self.pinv[i] = self.pinv[i + 1] * base % mod

    def hash(self, v):
        assert all(x != 0 for x in v)
        N = len(v)
        h = [0 for _ in range(N)]
        h[0] = v[0] % self.mod
        for i in range(1, N):
            h[i] = (h[i - 1] + v[i] * self.powr[i] % self.mod) % self.mod
        return h

    def range(self, h, l, r):  # [l, r)
        if r == 0:
            return 0
        elif l == 0:
            return h[r - 1]
        else:
            return (h[r - 1] - h[l - 1]) % self.mod * self.pinv[l] % self.mod


HR1 = PolyHash(500_000, base=31, mod=1_000_000_007)
HR2 = PolyHash(500_000, base=29, mod=1_000_000_009)

N = int(input())
S = [input() for _ in range(N)]
H1 = [HR1.hash([ord(c) for c in s]) for s in S]
H2 = [HR2.hash([ord(c) for c in s]) for s in S]

C = defaultdict(int)
for s, h1, h2 in zip(S, H1, H2):
    for i in range(len(s) + 1):
        C[(HR1.range(h1, 0, i), HR2.range(h2, 0, i))] += 1

for s, h1, h2 in zip(S, H1, H2):
    lb, ub = 0, len(s) + 1
    while ub - lb > 1:
        m = (lb + ub) // 2
        if C[(HR1.range(h1, 0, m), HR2.range(h2, 0, m))] >= 2:
            lb = m
        else:
            ub = m
    print(lb)

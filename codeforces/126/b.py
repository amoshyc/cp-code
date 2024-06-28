class PolyHash:
    def __init__(self, N, base=26, mod=10 ** 9 + 7):
        self.base = base
        self.mod = mod
        self.powr = [1 for _ in range(N)]
        self.pinv = [1 for _ in range(N)]
        for i in range(1, N):
            self.powr[i] = self.powr[i - 1] * base % mod
        self.pinv[-1] = pow(self.powr[-1], mod - 2, mod)
        for i in range(N - 2, -1, -1):
            self.pinv[i] = self.pinv[i + 1] * base % mod

    def transform(self, v):
        N = len(v)
        h = [0 for _ in range(N)]
        h[0] = v[0] % self.mod
        for i in range(1, N):
            h[i] = (h[i - 1] + v[i] * self.powr[i] % self.mod) % self.mod
        return h

    def substr(self, h, l, r):
        if l == 0:
            return h[r - 1]
        else:
            return (h[r - 1] - h[l - 1]) % self.mod * self.pinv[l] % self.mod


def solve():
    S = input()
    N = len(S)
    tool = PolyHash(N, 26, 10 ** 9 + 7)
    h = tool.transform([ord(c) - ord('a') for c in S])

    cands = []
    for length in range(1, N):
        prefix = tool.substr(h, 0, length)
        suffix = tool.substr(h, N - length, N)
        if prefix == suffix:
            cands.append((length, prefix))

    if len(cands) == 0:
        return "Just a legend"

    def check(m):
        length, val = cands[m]
        for i in range(1, N - length):
            if tool.substr(h, i, i + length) == val:
                return True
        return False
    
    lb, ub = 0, len(cands) - 1
    if check(lb) == False:
        return "Just a legend"
    if check(ub) == True:
        length, _ = cands[ub]
        return S[:length]

    while ub - lb > 1:
        m = (lb + ub) // 2
        if check(m) == True:
            lb = m
        else:
            ub = m
    
    length, _ = cands[lb]
    return S[:length]

print(solve())
'''
dp[i] = number of way to go from cell 0 to cell i
dp[0] = 1
for i in range(1, N):
    dp[i] = sum(dp[i - d] for d in seg for seg in segs)
Store dp in BIT
'''

mod = 998244353


class BIT:
    '''
    All methods use 0-based index while the internal use 1-based index
    '''

    def __init__(self, N, val=0):
        self.data = [val] * (N + 1)
        self.N = N

    def add_at(self, idx, val):
        i = idx + 1
        while i <= self.N:
            self.data[i] += val
            self.data[i] %= mod
            i += i & (-i)

    def sum_to(self, idx):
        res = 0
        i = idx + 1
        while i > 0:
            res += self.data[i]
            res %= mod
            i -= i & (-i)
        return res

    def range_sum(self, l, r):  # [l, r]
        return ((self.sum_to(r) - self.sum_to(l - 1)) + mod) % mod

    def __str__(self):
        data = [self.range_sum(i, i) for i in range(self.N)]
        return '[ ' + ' '.join(map(str, data)) + ' ]'


N, K = map(int, input().split())
segs = [list(map(int, input().split())) for _ in range(K)]

bit = BIT(N, 0)
bit.add_at(0, 1)
for i in range(N):
    for (l, r) in segs:
        lb = max(i - r, 0)
        ub = i - l
        if ub >= 0:
            val = bit.range_sum(lb, ub)
            bit.add_at(i, val)
print(bit.range_sum(N - 1, N - 1))


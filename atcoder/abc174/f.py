
class BIT:
    '''
    All methods use 0-based index while the implementation use 1-based index
    '''
    def __init__(self, N, val=0):
        self.data = [val] * (N + 1)
        self.N = N
    
    def add_at(self, idx, val):
        i = idx + 1
        while i <= self.N:
            self.data[i] += val
            i += i & (-i)
    
    def sum_to(self, idx):
        res = 0
        i = idx + 1
        while i > 0:
            res += self.data[i]
            i -= i & (-i)
        return res
    
    def sum_in(self, l, r): # [l, r]
        return self.sum_to(r) - self.sum_to(l - 1)
    
    
class Query:
    def __init__(self, i, l, r):
        self.i = i
        self.l = l
        self.r = r
    
    
N, Q = map(int, input().split())
C = list(map(int, input().split()))
    
queries = [[] for _ in range(N)]
for i in range(Q):
    l, r = map(int, input().split())
    l, r = l - 1, r - 1
    queries[r].append(Query(i, l, r))
    
bit = BIT(N)
last = [-1] * (max(C) + 1)
    
query_idx = 0
ans = [-1] * Q
for i, c in enumerate(C):
    if last[c] != -1:
        bit.add_at(last[c], -1)    
    bit.add_at(i, +1)
    last[c] = i
    
    for q in queries[i]:
        ans[q.i] = bit.sum_in(q.l, q.r)
    
print('\n'.join(map(str, ans)))
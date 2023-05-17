from collections import defaultdict
from bisect import bisect_right

N = int(input())
A = [int(x) for x in input().split()]
S = sorted(list(set(A)))

ans = defaultdict(int)
for a in A:
    cnt = len(S) - bisect_right(S, a)
    ans[cnt] += 1

for k in range(N):
    print(ans[k])

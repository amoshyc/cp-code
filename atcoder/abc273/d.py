from bisect import bisect_left, bisect_right
from collections import defaultdict


H, W, sr, sc = map(int, input().split())
sr, sc = sr - 1, sc - 1
pH = defaultdict(lambda: [-1, W])
pV = defaultdict(lambda: [-1, H])

N = int(input())
for _ in range(N):
    r, c = map(int, input().split())
    r, c = r - 1, c - 1
    pV[c].append(r)
    pH[r].append(c)

for pivots in pH.values():
    pivots.sort()
for pivots in pV.values():
    pivots.sort()

Q = int(input())
for _ in range(Q):
    d, l = input().split()
    l = int(l)

    if d == 'U':
        prev_r = pV[sc][bisect_left(pV[sc], sr) - 1]
        sr = max(sr - l, prev_r + 1)
    if d == 'D':
        next_r = pV[sc][bisect_right(pV[sc], sr)]
        sr = min(sr + l, next_r - 1)
    if d == 'L':
        prev_c = pH[sr][bisect_left(pH[sr], sc) - 1]
        sc = max(sc - l, prev_c + 1)
    if d == 'R':
        next_c = pH[sr][bisect_right(pH[sr], sc)]
        sc = min(sc + l, next_c - 1)
    
    print(sr + 1, sc + 1)

from bisect import bisect_left
from collections import defaultdict

N, M = map(int, input().split())
H = [int(x) for x in input().split()]
W = [int(x) for x in input().split()]

H.sort()
pref = defaultdict(int)
pref[0] = -H[0]
for i in range(1, N):
    if i % 2 == 1:
        pref[i] = pref[i - 1] + H[i]
    else:
        pref[i] = pref[i - 1] - H[i]

suff = defaultdict(int)
suff[N - 1] = H[N - 1]
for i in range(N - 2, -1, -1):
    if i % 2 == 1:
        suff[i] = suff[i + 1] - H[i]
    else:
        suff[i] = suff[i + 1] + H[i]

ans = 10**9
for w in W:
    idx = bisect_left(H, w)
    if idx == N:
        val = pref[N - 2] + w - H[-1]
    elif idx == 0:
        val = suff[1] + H[0] - w
    elif idx % 2 == 1:
        val = pref[idx - 2] + suff[idx] + w - H[idx - 1]
    else:
        val = pref[idx - 1] + suff[idx + 1] + H[idx] - w
    ans = min(ans, val)
print(ans)
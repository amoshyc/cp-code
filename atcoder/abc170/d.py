import numpy as np

N = int(input())
A = list(map(int, input().split()))
A = np.int32(A)

vals, cnts = np.unique(A, return_counts=True)

if len(vals) == 1:
    print(0 if cnts[0] > 1 else 1)
elif vals[0] == 1:
    print(0 if cnts[0] > 1 else 1)
else:
    sat = np.ones(vals.max() + 1, dtype=int)
    for x in vals:
        sat[2 * x::x] = 0
    ans = ((sat[vals] == 1) & (cnts == 1)).sum().item()
    print(ans)

# 5
# 2 2 2 3 3
# 5
# 2 2 2 4 4
# 5
# 1 1 1 1 2
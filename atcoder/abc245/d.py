import numpy as np

N, M = map(int, input().split())
A = [int(x) for x in input().split()]
C = [int(x) for x in input().split()]

A = np.int64(A)[::-1]
C = np.int64(C)[::-1]
B, _ = np.polydiv(C, A)
B = B[::-1].astype(int).tolist()
print(' '.join(map(str, B)))
import numpy as np
from itertools import combinations

def solve():
    H1, W1 = map(int, input().split())
    A = [[int(x) for x in input().split()] for _ in range(H1)]
    H2, W2 = map(int, input().split())
    B = [[int(x) for x in input().split()] for _ in range(H2)]

    A, B = np.int64(A), np.int64(B)

    for idx_r in combinations(range(H1), H2):
        for idx_c in combinations(range(W1), W2):
            AA = A[idx_r, :][:, idx_c]
            if np.all(AA == B):
                return True

    return False

print('Yes' if solve() else 'No')
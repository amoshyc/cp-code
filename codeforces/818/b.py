from collections import defaultdict

N, M = map(int, input().split())
L = list(map(int, input().split()))
A = [-1 for _ in range(N)]
U = defaultdict(bool)

def solve():
    for i in range(M - 1):
        cur = L[i] - 1
        nxt = L[i + 1] - 1
        delta = (nxt - cur) % N if nxt != cur else N
        if A[cur] == -1 or A[cur] == delta:
            A[cur] = delta
            U[delta] = True
        else:
            return False
    
    RA = [i for i, a in enumerate(A) if a == -1] # unfill positions
    RU = [i for i in range(1, N + 1) if not U[i]] # unused numbers
    if len(RA) != len(RU):
        return False

    for i, j in zip(RA, RU):
        A[i] = j
        U[j] = True

    return True

if not solve():
    print('-1')
else:
    print(' '.join(map(str, A)))

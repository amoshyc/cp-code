def polydiv_int(A, B):
    '''Return Q, R where A = B Q + R
    A: List of coef, where A = A[0] * x^0 + A[1] * x^1 + ... + A[N] * x^N
    B: List of coef, where B = B[0] * x^0 + B[1] * x^1 + ... + B[M] * x^M, B[M] != 0
    Q: List of coef, where Q = Q[0] * x^0 + Q[1] * x^1 + ... + Q[N - M] * x^(N - M)
    R: List of coef, where R = R[0] * x^0 + R[1] * x^1 + ... + R[N] * x^N
    '''
    N = len(A) - 1
    M = len(B) - 1
    if M > N:
        return [0], A
    degQ = N - M
    Q = [0 for _ in range(degQ + 1)]
    A = A.copy()
    for pivot in range(degQ, -1, -1):
        Q[pivot] = A[M + pivot] // B[M]
        for i in range(M + 1):
            A[pivot + i] -= Q[pivot] * B[i]
    return Q, A


N, M = map(int, input().split())
A = [int(x) for x in input().split()]
C = [int(x) for x in input().split()]
Q, R = polydiv_int(C, A)
print(' '.join(map(str, Q)))
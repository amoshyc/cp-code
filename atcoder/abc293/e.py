# Let S[i] = 1 + A + ... + A^i
# [ S[i] ]  = [A  1]^i  [S[0]]
# [ 1    ]  = [0  1]    [1   ]
# S[0] = 1
# answer = S[X - 1]

def matmul(A, B, mod):
    N, M = len(A), len(B[0])
    C = [[0 for _ in range(M)] for _ in range(N)]
    for r in range(N):
        for c in range(M):
            for i in range(len(A[0])):
                C[r][c] += A[r][i] * B[i][c]
                C[r][c] %= mod
    return C

def powmod(mat, exp, mod):
    N = len(mat)
    ans = [[0 for _ in range(N)] for _ in range(N)]
    for i in range(N):
        ans[i][i] = 1
    base = mat
    while exp:
        if exp & 1:
            ans = matmul(ans, base, mod)
        base = matmul(base, base, mod)
        exp >>= 1
    return ans


A, X, M = map(int, input().split())
mat = [[A, 1], [0, 1]]
mat = powmod(mat, X - 1, M)
ans = mat[0][0] * 1 + mat[0][1] * 1
print(ans % M)


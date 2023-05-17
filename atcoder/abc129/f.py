def matmul(A, B, mod):
    N, D, M = len(A), len(A[0]), len(B[0])
    C = [[0 for _ in range(M)] for _ in range(N)]
    for r in range(N):
        for c in range(M):
            for i in range(D):
                C[r][c] += A[r][i] * B[i][c] % mod
                C[r][c] %= mod
    return C

def powmat(A, p, mod):
    N = len(A)
    ans = [[0 for _ in range(N)] for _ in range(N)]
    for i in range(N):
        ans[i][i] = 1
    base = A
    while p:
        if p & 1:
            ans = matmul(base, ans, mod)
        base = matmul(base, base, mod)
        p >>= 1
    return ans


L, A, B, M = map(int, input().split())
pivots = [(10 ** d - A - 1) // B for d in range(1, 20)]

ans = [[A], [A], [1]] # t0
prev_pos = 0
for i in range(len(pivots)):
    if pivots[i] < 0:
        continue
    curr_pos = min(pivots[i], L - 1)
    cnt = curr_pos - prev_pos
    mat = powmat([[pow(10, i + 1, M), 1, B], [0, 1, B], [0, 0, 1]], cnt, M)
    ans = matmul(mat, ans, M)
    prev_pos = curr_pos
    if curr_pos >= L - 1:
        break
print(ans[0][0])
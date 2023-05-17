N, L, R = map(int, input().split())
A = [int(x) for x in input().split()]

suff = [0 for _ in range(N)]
suff[-1] = A[-1]
for i in range(N - 2, -1, -1):
    suff[i] = suff[i + 1] + A[i]
suff.append(0)

cands = [R * (N - j) - suff[j] for j in range(N + 1)]
cands_mn = cands.copy()
for i in range(N - 1, -1, -1):
    cands_mn[i] = min(cands_mn[i + 1], cands[i])

ans = min(L * N, R * N, sum(A))
for i in range(-1, N - 1):
    val_i = L * (i + 1) + suff[i + 1]
    # val_j = min(R * (N - j) - suff[j] for j in range(i + 1, N))
    val_j = cands_mn[i + 1]
    ans = min(ans, val_i + val_j)

print(ans)
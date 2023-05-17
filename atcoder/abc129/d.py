N, M = map(int, input().split())
A = [input() for _ in range(N)]

cntT = [[0 for _ in range(M)] for _ in range(N)]
cntD = [[0 for _ in range(M)] for _ in range(N)]
cntL = [[0 for _ in range(M)] for _ in range(N)]
cntR = [[0 for _ in range(M)] for _ in range(N)]

# init
for c in range(M):
    cntT[0][c] = 1 if A[0][c] == '.' else 0
    cntD[-1][c] = 1 if A[-1][c] == '.' else 0
for r in range(N):
    cntL[r][0] = 1 if A[r][0] == '.' else 0
    cntR[r][-1] = 1 if A[r][-1] == '.' else 0

# dp
for r in range(1, N):
    for c in range(M):
        cntT[r][c] = (cntT[r - 1][c] + 1) if A[r][c] == '.' else 0
for r in range(N - 2, -1, -1):
    for c in range(M):
        cntD[r][c] = (cntD[r + 1][c] + 1) if A[r][c] == '.' else 0
for r in range(N):
    for c in range(1, M):
        cntL[r][c] = (cntL[r][c - 1] + 1) if A[r][c] == '.' else 0
for r in range(N):
    for c in range(M - 2, -1, -1):
        cntR[r][c] = (cntR[r][c + 1] + 1) if A[r][c] == '.' else 0

# ans
ans = 0
for r in range(N):
    for c in range(M):
        if A[r][c] == '.':
            cnt = cntT[r][c] + cntD[r][c] + cntL[r][c] + cntR[r][c] - 4 + 1
            ans = max(ans, cnt)

print(ans)

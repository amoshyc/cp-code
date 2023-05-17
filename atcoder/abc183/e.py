H, W = map(int, input().split())
mod = 10 ** 9 + 7
S = [input() for _ in range(H)]

dp = [[0 for _ in range(W)] for _ in range(H)]
cntR = [0 for _ in range(H)] # row
cntC = [0 for _ in range(W)] # col
cntD = [0 for _ in range(H + W - 1)] # diagonal

for r in range(H):
    for c in range(W):
        if S[r][c] == '#':
            cntR[r] = 0
            cntC[c] = 0
            cntD[H - 1 - r + c] = 0
            continue

        if r == 0 and c == 0:
            dp[r][c] = 1
        else:
            if r > 0:
                dp[r][c] += cntC[c]
            if c > 0:
                dp[r][c] += cntR[r]
            if r > 0 and c > 0:
                dp[r][c] += cntD[H - 1 - r + c]
            dp[r][c] %= mod
        
        cntR[r] = (cntR[r] + dp[r][c]) % mod
        cntC[c] = (cntC[c] + dp[r][c]) % mod
        cntD[H - 1 - r + c] = (cntD[H - 1 - r + c] + dp[r][c]) % mod

print(dp[H - 1][W - 1])

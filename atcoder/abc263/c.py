N, M = map(int, input().split())
seq = [-1 for _ in range(N)]

def dfs(i, p):
    if i == N:
        print(' '.join(map(str, seq)))
    else:
        for v in range(p + 1, M + 1):
            seq[i] = v
            dfs(i + 1, seq[i])

dfs(0, 0)
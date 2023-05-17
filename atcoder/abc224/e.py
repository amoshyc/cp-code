from collections import defaultdict


H, W, N = map(int, input().split())
R = [0 for _ in range(N)]
C = [0 for _ in range(N)]
clusters = defaultdict(list)
for i in range(N):
    r, c, a = map(int, input().split())
    R[i] = r - 1
    C[i] = c - 1
    clusters[a].append(i)
clusters = [clusters[k] for k in sorted(list(clusters.keys()), reverse=True)]
# print(clusters)

# dp[i] = max(dp[j] | (r[i], c[i]) -> (r[j], c[j]))
# dp[i] = max(max_row[r[i]], max_col[c[i]]) + 1

max_row = [-1 for _ in range(H)]
max_col = [-1 for _ in range(W)]
dp = [-1 for _ in range(N)]

for indices in clusters:
    for i in indices:
        dp[i] = max(max_row[R[i]], max_col[C[i]]) + 1
    for i in indices:
        max_row[R[i]] = max(max_row[R[i]], dp[i])
        max_col[C[i]] = max(max_col[C[i]], dp[i])

print('\n'.join(map(str, dp)))
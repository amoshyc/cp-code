N, M = map(int, input().split())
inv = [[] for _ in range(M)]
for i in range(N):
    a, b = map(int, input().split())
    a, b = a - 1, b - 1
    inv[a].append(i)
    inv[b].append(i)

ends = [0 for _ in range(M + 2)]

r = 0
cnt = [0 for _ in range(N)]
n_covered = 0
for l in range(M):
    while r < M and n_covered < N:
        for pair_id in inv[r]:
            cnt[pair_id] += 1
            if cnt[pair_id] == 1:
                n_covered += 1
        r += 1
    
    if n_covered == N:
        min_len = r - l
        max_len = M - l
        ends[min_len] += 1
        ends[max_len + 1] -= 1

    for pair_id in inv[l]:
        cnt[pair_id] -= 1
        if cnt[pair_id] == 0:
            n_covered -= 1

ans = [0 for _ in range(M + 1)]
ans[1] = ends[1]
for i in range(2, M + 1):
    ans[i] = ans[i - 1] + ends[i]
print(' '.join(map(str, ans[1:])))

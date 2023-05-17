N = int(input())
lr = []
xs = []
for _ in range(N):
    a, b = map(int, input().split())
    lr.append((a, a + b))
    xs.append(a)
    xs.append(a + b)

xs = sorted(list(set(xs)))
vmap = {x : i for i, x in enumerate(xs)}
vinv = {i : x for i, x in enumerate(xs)}

A = [0 for _ in range(len(xs))]
for l, r in lr:
    A[vmap[l]] += 1
    A[vmap[r]] -= 1

ans = [0 for _ in range(N + 1)]
cnt = 0
for i in range(len(A) - 1):
    cnt = cnt + A[i]
    day = vinv[i + 1] - vinv[i]
    ans[cnt] += day

print(' '.join(map(str, ans[1:])))
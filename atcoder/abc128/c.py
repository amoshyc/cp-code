N, M = map(int, input().split())

A = [[] for _ in range(M)]
for i in range(M):
    _, *ss = map(int, input().split())
    A[i] = [s - 1 for s in ss]
P = [int(x) for x in input().split()]

ans = 0
for switch_bits in range(1 << N):
    cnt = 0
    for i in range(M):
        if sum(1 for s in A[i] if (switch_bits & (1 << s))) % 2 == P[i]:
            cnt += 1
    if cnt == M:
        ans += 1
print(ans)

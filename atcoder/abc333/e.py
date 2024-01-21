N = int(input())
t = []
x = []
for _ in range(N):
    a, b = map(int, input().split())
    t.append(a)
    x.append(b)

need = [0 for _ in range(N + 1)]
buy = [0 for _ in range(N)]
for i in range(N - 1, -1, -1):
    if t[i] == 2:
        need[x[i]] += 1
    else:
        if need[x[i]] > 0:
            buy[i] = 1
            need[x[i]] -= 1

own = [0 for _ in range(N + 1)]
cnt = 0
ans = 0
for i in range(N):
    if t[i] == 2:
        own[x[i]] -= 1
        cnt -= 1
        if own[x[i]] < 0:
            print(-1)
            exit()
    else:
        if buy[i]:
            own[x[i]] += 1
            cnt += 1
            ans = max(ans, cnt)

print(ans)
print(' '.join([str(buy[i]) for i in range(N) if t[i] == 1]))

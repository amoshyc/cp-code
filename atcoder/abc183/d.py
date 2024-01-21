N, W = map(int, input().split())

events = []
for _ in range(N):
    s, t, p = map(int, input().split())
    events.append((s, +p))
    events.append((t, -p))

events.sort()
cum_sum = 0
for (t, p) in events:
    cum_sum += p
    if cum_sum > W:
        print('No')
        break
else:
    print('Yes')

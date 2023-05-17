N, W = map(int, input().split())

events = []
for _ in range(N):
    s, t, p = map(int, input().split())
    events.append((s, +p))
    events.append((t, -p))

events.sort()
cum_sum = 0
max_need = -1
for (t, p) in events:
    cum_sum += p
    max_need = max(max_need, cum_sum)

print('Yes' if max_need <= W else 'No')

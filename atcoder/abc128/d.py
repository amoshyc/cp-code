N, K = map(int, input().split())
V = [int(x) for x in input().split()]

ans = 0
for pivot1 in range(N):
    for pivot2 in range(pivot1, N + 1):
        data = V[:pivot1] + V[pivot2:]
        data = sorted(data)
        total = sum(data)
        for n_remove in range(len(data)):
            n_operation = pivot1 + N - pivot2 + n_remove
            if n_operation <= K:
                ans = max(ans, total)
            total -= data[n_remove]
print(ans)

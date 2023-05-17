N = int(input())

E = [0 for _ in range(N + 1)]
E[N] = 3.5
for i in range(N - 1, 0, -1):
    val = 0.0
    for j in range(1, 7):
        val += j if j >= E[i + 1] else E[i + 1]
    E[i] = val / 6
print(f'{E[1]:.10f}')
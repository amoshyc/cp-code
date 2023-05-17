def f(x, a_min, a_max, b_min, b_max):
    if x < a_min + b_min or x > a_max + b_max:
        return 0
    lb_b = max(x - a_max, b_min)
    ub_b = min(x - a_min, b_max)
    return ub_b - lb_b + 1


N, K = map(int, input().split())

ans = 0
for beta in range(2, 2 * N + 1):
    alpha = beta + K
    if 1 <= alpha <= 2 * N:
        ans += f(alpha, 1, N, 1, N) * f(beta, 1, N, 1, N)
print(ans)

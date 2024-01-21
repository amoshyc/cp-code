from math import floor, ceil

def solve(X, D, lb, ub):
    # min abs(X + t * D) (lb <= t <= ub) (lb, ub, t are int)
    ts = [
        lb,
        ub,
        max(min(floor(-X / D), ub), lb),
        max(min(ceil(-X / D), ub), lb),
    ]
    return min([abs(X + t * D) for t in ts])

X, K, D = map(int, input().split())
if K % 2 == 0:
    ans = solve(X, D * 2, -(K // 2), +(K // 2))
else:
    ans1 = solve(X + D, D * 2, -((K + 1) // 2), +((K - 1)) // 2)
    ans2 = solve(X - D, D * 2, -((K - 1) // 2), +((K + 1)) // 2)
    ans = min(ans1, ans2)
print(ans)


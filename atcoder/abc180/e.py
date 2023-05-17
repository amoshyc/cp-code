def dist(city1, city2):
    a, b, c = city1
    p, q, r = city2
    return abs(a - p) + abs(b - q) + max(0, r - c)


N = int(input())
cities = [[int(x) for x in input().split()] for _ in range(N)]

inf = 10 ** 15
dp = [[inf for _ in range(N)] for _ in range(1 << N)]

dp[1 << 0][0] = 0

for s in range(1, 1 << N):
    for v in range(N):
        if s & (1 << v):
            for u in range(N):
                if s & (1 << u):
                    val = dp[s ^ (1 << v)][u] + dist(cities[u], cities[v])
                    dp[s][v] = min(dp[s][v], val)

ans = min([dp[-1][v] + dist(cities[v], cities[0]) for v in range(N)])
print(ans)

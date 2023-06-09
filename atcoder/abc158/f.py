import numpy as np
import collections

Robot = collections.namedtuple('Robot', ['l', 'r'])

N = int(input())
MOD = 998244353
robots = []
for _ in range(N):
    x, d = map(int, input().split())
    robots.append(Robot(x, x + d))
robots.sort()

robots.append(Robot(10**10, 10**10 + 1))
stack = [N]

dp = np.zeros(N + 1, dtype=int)
dp[N] = 1

for i in range(N - 1, -1, -1):
    # For each i, we want the stack contains all the j such that
    # robots[j] is not affected by robot[i], therefore
    # we remove those robots that are intersected with robot[i]
    while robots[stack[-1]].l < robots[i].r:
        stack.pop()

    # dp[i] = dp[i + 1] (not activate robot i) + dp[j] (activate robot i)
    j = stack[-1]
    dp[i] = (dp[i + 1] + dp[j]) % MOD
    stack.append(i)

print(dp[0])
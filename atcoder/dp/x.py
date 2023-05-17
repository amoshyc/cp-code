from collections import namedtuple

Block = namedtuple('Block', ['w', 's', 'v'])

N = int(input())
blocks = [Block(*map(int, input().split())) for _ in range(N)]
blocks.sort(key=lambda block: block.w + block.s, reverse=True)

# dp[i, j] = maximum total value if [0, i) blocks are considered and the top solidness is j
dp = [[0 for _ in range(20001)] for _ in range(N + 1)]

for i in range(N):
    for j in range(20001):
        w, s, v = blocks[i]
        # Not use block i
        dp[i + 1][j] = max(dp[i + 1][j], dp[i][j])
        # Use block i
        if j >= w:
            dp[i + 1][min(s, j - w)] = max(dp[i + 1][min(s, j - w)], dp[i][j] + v)
print(max(dp[N]))

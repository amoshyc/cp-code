# 7r + c + 1
# 0 <= r < 10**100
# 0 <= c < 7

def solve():
    N, M = map(int, input().split())
    B = [[int(x) for x in input().split()] for _ in range(N)]

    for c0 in range(7 - M + 1):
        r0, m = divmod(B[0][0] - c0 - 1, 7)
        if m != 0:
            continue
        if r0 < 0 or r0 + N - 1 >= 10 ** 100:
            continue
        if all(B[r][c] == 7 * (r + r0) + (c + c0) + 1 for r in range(N) for c in range(M)):
            return 'Yes'
    
    return 'No'

print(solve())
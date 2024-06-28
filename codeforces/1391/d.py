def is_valid(prev_mask, curr_mask, N):
    bits = prev_mask ^ curr_mask
    if N == 2:
        return bits != 0b00 and bits != 0b11
    else:
        check1 = bits != 0b000 and bits != 0b001 and bits != 0b100
        check2 = bits != 0b111 and bits != 0b110 and bits != 0b011
        return check1 and check2


def solve():
    N, M = map(int, input().split())
    A = [list(map(int, input())) for _ in range(N)]

    if N >= 4 and M >= 4:
        return -1

    if N == 1 or M == 1:
        return 0

    state = [0] * M
    for c in range(M):
        for r in range(N):
            if A[r][c] == 1:
                state[c] |= 1 << (N - 1 - r)

    popcounts = [bin(x).count('1') for x in range(1 << N)]

    dp = [[N * M] * (1 << N) for _ in range(M)]

    for mask in range(1 << N):
        dp[0][mask] = popcounts[state[0] ^ mask]

    for i in range(1, M):
        for curr_mask in range(1 << N):
            dp[i][curr_mask] = min(
                [
                    dp[i - 1][prev_mask]
                    for prev_mask in range(1 << N)
                    if is_valid(prev_mask, curr_mask, N)
                ]
            ) + popcounts[state[i] ^ curr_mask]

    return min(dp[-1])


print(solve())

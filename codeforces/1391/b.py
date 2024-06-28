TC = int(input())
for tc in range(TC):
    N, M = map(int, input().split())
    A = [input() for _ in range(N)]

    ans = 0
    for r in range(N - 1):
        if A[r][-1] != 'D':
            ans += 1
    for c in range(M - 1):
        if A[-1][c] != 'R':
            ans += 1
    print(ans)

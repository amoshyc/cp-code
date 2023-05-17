def build_prefix(A):
    P = [A[0]]
    for i in range(1, len(A)):
        P.append(P[-1] + A[i])
    return P


def query(P, i, j):
    if j - i <= 0:
        return 0
    if i == 0:
        return P[j - 1]
    else:
        return P[j - 1] - P[i - 1]


def solve():
    N, K = map(int, input().split())
    S = input()

    P1 = build_prefix([int(c == '1') for c in S])
    PQ = build_prefix([int(c == '?') for c in S])

    ans = 0
    for s in range(N - K + 1):
        t = s + K # [s, t)

        check1 = query(P1, s, t) + query(PQ, s, t) == K
        check2 = query(P1, 0, s) == 0 and query(P1, t, N) == 0

        if all([check1, check2]):
            ans += 1

    print('Yes' if ans == 1 else 'No')


T = int(input())
for _ in range(T):
    solve()

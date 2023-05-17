N, L, R = map(int, input().split())
N = bin(N)[2:][::-1]
N = [int(d) for d in N]

def f(ub):
    ub = bin(ub)[2:][::-1]
    ub = [int(d) for d in ub]
    W = len(ub)

    while len(N) < W:
        N.append(0)

    dp = [[[0, 0], [0, 0]] for _ in range(W + 1)]
    dp[0][0][1] = 1
    for i in range(W):  # digit
        for f1 in range(2):  # larger than ub
            for f2 in range(2):  # smaller than N
                for c in range(2):
                    new_f1 = (c > ub[i]) or (c == ub[i] and f1 == 1)
                    new_f2 = (c ^ N[i] < N[i]) or (c ^ N[i] == N[i] and f2 == 1)
                    dp[i + 1][new_f1][new_f2] += dp[i][f1][f2]

    return dp[W][0][1]


print(f(R) - f(L - 1))

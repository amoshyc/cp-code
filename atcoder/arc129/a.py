def f(n, ub):
    ub = bin(ub)[2:][::-1]
    ub = [int(d) for d in ub]
    W = len(ub)

    nn = bin(n)[2:].rjust(W, '0')[::-1]
    nn = [int(d) for d in nn]

    dp = [[[0, 0], [0, 0]] for _ in range(W + 1)]
    dp[0][0][1] = 1
    for i in range(W):  # digit
        for f1 in range(2):  # larger than ub
            for f2 in range(2):  # smaller than N
                for c in range(2):
                    new_f1 = (c > ub[i]) or (c == ub[i] and f1 == 1)
                    new_f2 = (c ^ nn[i] < nn[i]) or (c ^ nn[i] == nn[i] and f2 == 1)
                    dp[i + 1][new_f1][new_f2] += dp[i][f1][f2]

    return dp[W][0][1]


N, L, R = map(int, input().split())
print(f(N, R) - f(N, L - 1))

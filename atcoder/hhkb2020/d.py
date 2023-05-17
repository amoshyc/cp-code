TC = int(input())
for _ in range(TC):
    N, A, B = map(int, input().split())
    M = 10 ** 9 + 7

    if A + B > N:
        print(0)
    else:
        X4 = (N - A - B + 2) * (N - A - B + 1) // 2
        X3 = 2 * X4 % M
        X2 = (N - A + 1) * (N - B + 1) % M - X3 % M + M
        X1 = X2 ** 2 % M
        ans = (N - A + 1) ** 2 % M * (N - B + 1) ** 2 % M - X1 + M
        print(ans % M)
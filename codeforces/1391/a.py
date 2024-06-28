TC = int(input())

for _ in range(TC):
    N = int(input())

    ans = [-1] * N
    lb, ub = 1, N
    for i in range(N):
        if i % 2 == 0:
            ans[i] = ub
            ub -= 1
        else:
            ans[i] = lb
            lb += 1
    print(' '.join(map(str, ans)))
    
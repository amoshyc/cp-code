import numpy as np


def solve():
    N = int(input())
    A = list(map(int, input().split()))

    A = np.int32(A)
    D = np.diff(A)
    if np.all(D <= 0):
        return 1000
    if np.all(D >= 0):
        stock, money = divmod(1000, A[0])
        money = stock * A[-1] + money
        return money

    money = 1000
    s = 0
    while s < N - 1:
        t = s + 1
        if A[t] >= A[s]:
            while t < N and A[t] >= A[t - 1]:
                t += 1
            stock, money = divmod(money, A[s])
            money = stock * A[t - 1] + money
            # print('++:', A[s:t], money)
            s = t
        else:
            while t < N and A[t] < A[t - 1]:
                t += 1
            # print('--:', A[s:t], money)
            s = t - 1
    
    return money


print(solve())

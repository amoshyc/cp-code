from collections import deque


def solve():
    M, K = map(int, input().split())

    if M == 1 and K != 0:
        return '-1'

    if K >= pow(2, M):
        return '-1'

    if K == 0:
        A = []
        for x in range(2 ** M):
            A.append(x)
            A.append(x)
        return ' '.join(map(str, A))

    A = deque([K])
    for val in range(2 ** M):
        if val != K:
            A.append(val)
            A.appendleft(val)
    A.append(K)
    
    return ' '.join(map(str, A))

print(solve())

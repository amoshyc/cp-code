from collections import deque


def solve():
    M, K = map(int, input().split())

    if M == 1:
        if K != 0:
            return '-1'
        return '0 0 1 1'

    if K >= pow(2, M):
        return '-1'

    A = deque([K])
    for val in range(2 ** M):
        if val != K:
            A.append(val)
            A.appendleft(val)
    A.append(K)
    
    return ' '.join(map(str, A))

print(solve())

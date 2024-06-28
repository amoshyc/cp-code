import io
import os

input = io.BytesIO(os.read(0, os.fstat(0).st_size)).readline


def solve():
    N, L = map(int, input().split())
    A = [int(x) for x in input().split()]
    A = [0] + A + [L]

    t1 = [0] * len(A)  # t1[i] = time for car1 to arrive at A[i]
    v1 = [1] * len(A)  # v1[i] = the speed of car1 from A[i] to A[i + 1]
    for i in range(1, len(A)):
        distance = A[i] - A[i - 1]
        t1[i] = t1[i - 1] + distance / v1[i - 1]
        v1[i] = v1[i - 1] + 1

    t2 = [0] * len(A)  # t2[i] = time for car2 to arrive at A[i]
    v2 = [1] * len(A)  # v2[i] = the speed of car2 from A[i] to A[i - 1]
    for i in range(len(A) - 2, -1, -1):
        distance = A[i + 1] - A[i]
        t2[i] = t2[i + 1] + distance / v2[i + 1]
        v2[i] = v2[i + 1] + 1

    # if they meet between A[i] and A[i + 1], then
    # (t - t1[i]) * v1[i] + (t - t2[i + 1]) * v2[i + 1] = A[i + 1] - A[i]
    # where t is the time they meet
    for i in range(len(A) - 1):
        if t1[i] <= t2[i] and t1[i + 1] >= t2[i + 1]:
            denom = v1[i] + v2[i + 1]
            nomin = A[i + 1] - A[i] + t1[i] * v1[i] + t2[i + 1] * v2[i + 1]
            return nomin / denom


TC = int(input())
for _ in range(TC):
    print('{:.8f}'.format(solve()))

from collections import defaultdict

N = int(input())
A = [int(x) for x in input().split()]

reset = None
adds = defaultdict(lambda: 0)

Q = int(input())
for _ in range(Q):
    a, b, *c = map(int, input().split())
    if a == 1:
        reset = b
        adds.clear()
    if a == 2:
        adds[b - 1] += c[0]
    if a == 3:
        if reset is None:
            print(A[b - 1] + adds[b - 1])
        else:
            print(reset + adds[b - 1])
def solve():
    N = int(input())
    A = list(map(int, input().split()))
    if A[0] + A[1] <= A[-1]:
        return '{} {} {}'.format(1, 2, N)
    return '-1'
        

TC = int(input())
for _ in range(TC):
    print(solve())
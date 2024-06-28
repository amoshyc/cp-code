def solve():
    N = int(input())
    A = [int(x) for x in input().split()]
    L = [int(x) for x in input().split()]

    values = sorted([a for a, l in zip(A, L) if l == 0])
    for i in range(N):
        if L[i] == 0:
            A[i] = values.pop()
    
    return ' '.join(map(str, A))

tc = int(input())
for _ in range(tc):
    print(solve())
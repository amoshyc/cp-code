def solve():
    n = int(input())
    A = [int(x) for x in input().split()]
    
    S = sum(A)
    if S == 0:
        return None
    
    A = sorted(A)
    return A if S < 0 else A[::-1]


TC = int(input())
for _ in range(TC):
    res = solve()
    if res is None:
        print('NO')
    else:
        print('YES')
        print(' '.join(map(str, res)))


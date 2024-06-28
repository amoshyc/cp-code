def solve():
    N = int(input())
    A = [int(x) for x in input().split()]
    B = [int(x) for x in input().split()]
    C = [int(x) for x in input().split()]
    
    P = [A[0]]
    for i in range(1, N - 1):
        for x in [A[i], B[i], C[i]]:
            if x != P[-1]:
                P.append(x)
                break
    for x in [A[-1], B[-1], C[-1]]:
        if x != P[0] and x != P[-1]:
            P.append(x)
            break
        
    return ' '.join(map(str, P))
    
    
TC = int(input())
for _ in range(TC):
    print(solve())
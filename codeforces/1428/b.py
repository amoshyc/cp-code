def solve():
    N = int(input())
    A = input()

    if all((a == '>' or a == '-' )for a in A):
        return N
    if all((a == '<' or a == '-') for a in A):
        return N
    
    cnt = 0
    for i in range(N):
        if A[i] == '-' or A[i - 1] == '-':
            cnt += 1
    return cnt

TC = int(input())
for _ in range(TC):
    print(solve())
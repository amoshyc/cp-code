def solve():
    N = int(input())
    A = [input() for _ in range(N)]

    for i in range(N):
        for j in range(i + 1, N):
            if A[i][j] == 'W' and A[j][i] != 'L':
                return False
            if A[i][j] == 'L' and A[j][i] != 'W':
                return False
            if A[i][j] == 'D' and A[j][i] != 'D':
                return False
                
    return True

print('correct' if solve() else 'incorrect')
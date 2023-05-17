A = [[0 for _ in range(15)] for _ in range(15)]

for i in [0, 2, 4, 6]:
    l = 7 - i
    r = 7 + i
    for x in range(l, r + 1):
        A[l][x] = 1
        A[x][r] = 1
        A[r][x] = 1
        A[x][l] = 1
# for row in A:
#     print(' '.join(map(str, row)))

R, C = map(int, input().split())
R, C = R - 1, C - 1
print('white' if A[R][C] == 1 else 'black')
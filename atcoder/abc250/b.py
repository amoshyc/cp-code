N, A, B = map(int, input().split())

res = [[' ' for _ in range(N * B)] for _ in range(N * A)]

for r in range(N):
    for c in range(N):
        k = '.' if (r + c) % 2 == 0 else '#'
        for i in range(A):
            for j in range(B):
                res[r * A + i][c * B + j] = k

for r in res:
    print(''.join(r))
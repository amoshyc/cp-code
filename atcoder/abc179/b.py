N = int(input())
A = [list(map(int, input().split())) for _ in range(N)]

for i in range(N - 2):
    if all(x == y for (x, y) in A[i:i+3]):
        print('Yes')
        break
else:
    print('No')
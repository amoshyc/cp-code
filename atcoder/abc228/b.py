N, X = map(int, input().split())
X -= 1
A = [int(x) - 1 for x in input().split()]
vis = [0 for _ in range(N)]

vis[X] = 1
while vis[A[X]] == 0:
    vis[A[X]] = 1
    X = A[X]

print(sum(vis))
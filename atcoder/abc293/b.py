N = int(input())
A = [int(x) - 1 for x in input().split()]
V = [False for _ in range(N)]

for i in range(N):
    if V[i] is False:
        V[A[i]] = True

ans = [i + 1 for i, v in enumerate(V) if v is False]
print(len(ans))
print(' '.join(map(str, ans)))
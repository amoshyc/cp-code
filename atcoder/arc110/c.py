N = int(input())
A = [int(x) for x in input().split()]

used = [False for _ in range(N)]
pos = {a: i for i, a in enumerate(A)}
ans = []

for i in range(N):
    while pos[i + 1] != i:
        # swap index pos[i + 1] and pos[i + 1] - 1
        j = pos[i + 1]
        if used[j]:
            print(-1)
            exit()
        else:
            used[j] = True
            pos[A[j]] -= 1
            pos[A[j - 1]] += 1
            A[j - 1], A[j] = A[j], A[j - 1]
            ans.append(j - 1)

if len(ans) == N - 1:
    print('\n'.join([f'{j + 1}' for j in ans]))
else:
    print(-1)

N = int(input())
A = [int(x) for x in input().split()]

ans = [A[0]]
for i in range(1, N):
    if A[i] > A[i - 1]:
        for j in range(A[i - 1] + 1, A[i]):
            ans.append(j)
    else:
        for j in range(A[i - 1] - 1, A[i], -1):
            ans.append(j)
    ans.append(A[i])
print(' '.join(map(str, ans)))
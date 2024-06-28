N = int(input())
A = list(map(int, input().split()))

ans = sum(1 for x in A if x == 0)
for i in range(N):
    if A[i] == 1:
        pref = sum(1 for x in A[:i] if x == 0)
        suff = sum(1 for x in A[i:] if x == 1)
        ans = max(ans, pref + suff)

print(ans)
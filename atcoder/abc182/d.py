N = int(input())
A = [int(x) for x in input().split()]

prev = 0
pref = 0
mx_pref = 0
ans = 0
for i in range(N):
    pref += A[i]
    mx_pref = max(mx_pref, pref)
    ans = max(ans, prev + mx_pref)
    prev += pref
print(ans)
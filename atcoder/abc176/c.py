N = int(input())
A = list(map(int, input().split()))

ans = 0
mx = A[0]
for i in range(1, N):
    if A[i] < mx:
        ans += mx - A[i]
    mx = max(mx, A[i])
print(ans)
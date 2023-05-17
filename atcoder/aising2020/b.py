N = int(input())
A = list(map(int, input().split()))
ans = sum(1 for a in A[::2] if a % 2 == 1)
print(ans)
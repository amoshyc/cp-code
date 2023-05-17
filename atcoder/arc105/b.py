from math import gcd


N = int(input())
A = [int(x) for x in input().split()]

res = A[0]
for i in range(1, N):
    res = gcd(res, A[i])
print(res)


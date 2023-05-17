import math

N = int(input())
ans = 0
for A in range(1, N):
    lb = 1
    ub = math.ceil(N / A) - 1
    # ub = N // A - int(N % A == 0)
    ans += ub - lb + 1
print(ans)
from statistics import median

N = int(input())
A, B = [], []
for _ in range(N):
    a, b = map(int, input().split())
    A.append(a)
    B.append(b)

mn = median(A)
mx = median(B)

if N % 2 == 1:
    print(int(mx - mn + 1))        # mn, mn + 1, mn + 2, ..., mx
else:
    print(int((mx - mn) * 2 + 1))    # mn, mn + 0.5, mn + 1, ..., mx
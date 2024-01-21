from itertools import combinations

N = input()
r = int(N) % 3
A = [int(c) % 3 for c in N]
if r == 0:
    print(0)
elif len(A) >= 2 and any(a == r for a in A):
    print(1)
elif len(A) >= 3 and any((a + b) % 3 == r for a, b in combinations(A, 2)):
    print(2)
else:
    print(-1)

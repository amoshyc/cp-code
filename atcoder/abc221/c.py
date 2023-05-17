from itertools import permutations

def f(A):
    if A[0] == 0:
        return 0
    val = 0
    for a in A:
        val = val * 10 + a
    return val

N = sorted([int(x) for x in input()])

ans = 0
for perm in permutations(N):
    for i in range(1, len(perm)):
        val1, val2 = f(perm[:i]), f(perm[i:])
        ans = max(ans, val1 * val2)

print(ans)
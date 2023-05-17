N = int(input())
S = [int(x) for x in input().split()]
V = max(S)

A = set()
for a in range(1, 1000 + 1):
    for b in range(1, 1000 + 1):
        A.add(4 * a * b + 3 * a + 3 * b)

print(len([x for x in S if x not in A]))
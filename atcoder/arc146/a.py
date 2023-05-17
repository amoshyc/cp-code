N = int(input())
S = [int(x) for x in input().split()]
S = sorted(S)[-3:]
S = [str(x) for x in S]
a, b, c = S

A = max([
    int(''.join([a, b, c])),
    int(''.join([a, c, b])),
    int(''.join([b, a, c])),
    int(''.join([b, c, a])),
    int(''.join([c, a, b])),
    int(''.join([c, b, a])), 
])

print(A)
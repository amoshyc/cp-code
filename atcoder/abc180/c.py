from math import sqrt, ceil

N = int(input())

factors = [1, N]
for factor in range(2, ceil(sqrt(N)) + 1):
    if N % factor == 0:
        factors.append(factor)
        factors.append(N // factor)

factors = list(set(factors))
print('\n'.join(map(str, sorted(factors))))
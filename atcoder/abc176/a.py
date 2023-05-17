from math import ceil

N, X, T = map(int, input().split())
print(ceil(N / X) * T)
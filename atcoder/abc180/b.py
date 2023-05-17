from math import sqrt

N = int(input())
X = [int(x) for x in input().split()]

print(sum(abs(x) for x in X))
print(sqrt(sum(x ** 2 for x in X)))
print(max(abs(x) for x in X))
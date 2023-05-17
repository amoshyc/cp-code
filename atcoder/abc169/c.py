from decimal import Decimal

A, B = [Decimal(x) for x in input().split()]
print(int(A * B))
from decimal import Decimal
n = int(input())
x = int((Decimal(8 * n + 9).sqrt() - 1) // 2)
print(n - x + 1)
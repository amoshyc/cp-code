from math import sqrt

x, y = map(int, input().split())
r = sqrt(x ** 2 + y ** 2)
print(f'{x / r:.10f} {y / r:.10f}')

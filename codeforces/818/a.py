import sys

for line in sys.stdin:
    n, k = map(int, line.split())

    d = n // (2 * (k + 1))
    c = k * d

    print(d, c, n - c - d)
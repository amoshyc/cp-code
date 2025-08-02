a, b = map(int, input().split())
ceil = (a + b - 1) // b
floor = a // b

if abs(floor * b - a) < abs(ceil * b - a):
    print(floor)
else:
    print(ceil)

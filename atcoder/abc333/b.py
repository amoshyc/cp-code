groups = [
    ["AB", "AE", "BC", "CD", "DE"],
    ["AC", "AD", "BD", "BE", "CE"],
]

a = list(input())
b = list(input())

if a[0] > a[1]:
    a[0], a[1] = a[1], a[0]

if b[0] > b[1]:
    b[0], b[1] = b[1], b[0]

x = 0 if "".join(a) in groups[0] else 1
y = 0 if "".join(b) in groups[0] else 1

print("Yes" if x == y else "No")

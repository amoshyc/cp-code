units = []
for i in range(1, 20):
    units.append(int("1" * i))

vals = set()
for x in units:
    for y in units:
        for z in units:
            vals.add(x + y + z)

vals = sorted(list(vals))
n = int(input())
print(vals[n - 1])

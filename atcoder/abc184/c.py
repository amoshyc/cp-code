def in_one_step(r1, c1, r2, c2):
    return r1 + c1 == r2 + c2 or r1 - c1 == r2 - c2 or abs(r1 - r2) + abs(c1 - c2) <= 3

r1, c1 = map(int, input().split())
r2, c2 = map(int, input().split())

if r1 == r2 and c1 == c2:
    print(0)
elif in_one_step(r1, c1, r2, c2):
    print(1)
elif (abs(r2 - r1) % 2 + abs(c2 - c1) % 2) % 2 == 0:
    print(2)
else:
    adjs = [(r, c) for r in range(r1 - 2, r1 + 3) for c in range(c1 - 2, c1 + 3)]
    adjs.append((r1 - 3, c1))
    adjs.append((r1 + 3, c1))
    adjs.append((r1, c1 - 3))
    adjs.append((r1, c1 + 3))
    if any(in_one_step(r, c, r2, c2) for (r, c) in adjs):
        print(2)
    else:
        print(3)
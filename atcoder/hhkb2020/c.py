N = int(input())
P = [int(x) for x in input().split()]

use = [False for _ in range(200000 + 10)]
val = 0

for p in P:
    if val != p:
        print(val)
        use[p] = True
        continue
    use[p] = True
    while use[val]:
        val += 1
    print(val)

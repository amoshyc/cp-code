from math import gcd

n = int(input())
lcm = 1
for i in range(2, n + 1):
    lcm = lcm * i // gcd(lcm, i)
print(lcm + 1)

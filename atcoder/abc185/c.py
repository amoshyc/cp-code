# x1 + x2 + ... + x12 = L (xi >= 0)
# x1' + x2' + ... + x12' = L - 12 (xi' > 0)
# => H(12, L - 12) = C(L - 1, L - 12) = C(L - 1, 11)
from math import comb
L = int(input())
print(comb(L - 1, 11))
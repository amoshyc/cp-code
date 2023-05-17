A, B = map(int, input().split())
C = 0
if (A & (1 << 0)) or (B & (1 << 0)):
    C += (1 << 0)
if (A & (1 << 1)) or (B & (1 << 1)):
    C += (1 << 1)
if (A & (1 << 2)) or (B & (1 << 2)):
    C += (1 << 2)
print(C)
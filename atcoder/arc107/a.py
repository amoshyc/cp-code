A, B, C = map(int, input().split())
A = (1 + A) * A // 2 % 998244353
B = (1 + B) * B // 2 % 998244353
C = (1 + C) * C // 2 % 998244353
print(A * B * C % 998244353)
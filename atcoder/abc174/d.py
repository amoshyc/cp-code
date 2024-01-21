N = int(input())
C = input()
T = 'R' * C.count('R') + 'W' * C.count('W')
ans = sum([1 for a, b in zip(C, T) if a != b]) // 2
print(ans)
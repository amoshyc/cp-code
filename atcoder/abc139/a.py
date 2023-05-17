S = input()
T = input()
cnt = sum(1 for a, b in zip(S, T) if a == b)
print(cnt)
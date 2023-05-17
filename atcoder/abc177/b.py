S = input()
T = input()

ans = len(T) + 1
for i in range(len(S) - len(T) + 1):
    val = sum(1 for a, b in zip(S[i:i+len(T)], T) if a != b)
    ans = min(ans,  val)
print(ans)
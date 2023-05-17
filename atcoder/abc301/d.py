S = input()
mapping = { '0': 0, '1': 1, '?': -1 }
A = [mapping[c] for c in S]
A.reverse()

N = int(input())
B = []
while N > 0:
    B.append(N & 1)
    N >>= 1

while len(A) < 62:
    A.append(0)
while len(B) < 62:
    B.append(0)
    
inf = 10 ** 18 + 1
dp = [[-inf, -inf] for _ in range(62 + 1)]
dp[0] = [0, 0]
for i in range(62):
    for f in range(2):
        cands = [0, 1] if A[i] == -1 else [A[i]]
        for j in cands:
            new_f = int(j > B[i] or (j == B[i] and f == 1))
            new_v = j * (1 << i) + dp[i][f]
            dp[i + 1][new_f] = max(dp[i + 1][new_f],  new_v)

if dp[-1][0] < 0:
    print(-1)
else:
    print(dp[-1][0])
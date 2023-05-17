S = input()
A = [S]
R = S
L = S
for i in range(len(S)):
    R = R[1:] + R[0]
    L = L[-1] + L[:-1]
    A.append(R)
    A.append(L)
A.sort()
print(A[0])
print(A[-1])
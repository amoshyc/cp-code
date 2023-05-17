N = int(input())
A = list(map(int, input().split()))

S = 0
for a in A:
    S = S ^ a

for a in A:
    print(S ^ a)
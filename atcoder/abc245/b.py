N = int(input())
A = [int(x) for x in input().split()]
A = set(A)

for i in range(2000 + 2):
    if i not in A:
        print(i)
        break
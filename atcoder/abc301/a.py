N = int(input())
A = input()

a = A.count('T')
b = A.count('A')

if a > b:
    print('T')
elif a < b:
    print('A')
else:
    print('T' if A[-1] == 'A' else 'A')
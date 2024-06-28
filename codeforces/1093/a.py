T = int(input())
for _ in range(T):
    n = int(input())
    if n == 2 or n == 3:
        print(1)
    else:
        print(n // 2)

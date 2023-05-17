X, Y = map(int, input().split())

b = (Y - 2 * X) // 2
a = X - b

if 0 <= a <= X and 0 <= b <= X and a + b == X and 2 * a + 4 * b == Y:
    print('Yes')
else:
    print('No')
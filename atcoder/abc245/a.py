A, B, C, D = map(int, input().split())
x = A * 60 * 60 + B * 60
y = C * 60 * 60 + D * 60 + 1
print('Takahashi' if x < y else 'Aoki')
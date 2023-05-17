N, X, Y = map(int, input().split())

R = 1
B = 0

for level in range(N, 1, -1):
    newR = 0
    newB = 0

    newR += R
    B += R * X

    newR += B
    newB += B * Y

    R = newR
    B = newB

print(B)
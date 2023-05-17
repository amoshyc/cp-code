N = int(input())
H = [int(x) for x in input().split()]

idx = 0
val = H[0]

for i in range(1, N):
    if H[i] > val:
        val = H[i]
        idx = i

print(idx + 1)
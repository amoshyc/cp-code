N = int(input())
H = list(map(int, input().split()))

s, t = 0, 1
max_len = -1
while s < N:
    t = s + 1
    while t < N and H[t] <= H[t - 1]:
        t += 1
    max_len = max(max_len, t - s)
    s = t
print(max_len - 1)
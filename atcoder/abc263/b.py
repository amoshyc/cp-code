N = int(input())
P = [-1] + [int(x) - 1 for x in input().split()]

cnt = 0
x = N - 1
while P[x] != -1:
    x = P[x]
    cnt += 1

print(cnt)

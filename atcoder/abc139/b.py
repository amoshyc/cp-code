a, b = map(int, input().split())

cnt = 1
add = 0
while cnt < b:
    cnt = (cnt - 1) + a
    add += 1
print(add)
N = int(input())

cnt = 0
for base in range(1, 10 ** 6 + 1):
    if int('{}{}'.format(base, base)) <= N:
        cnt += 1
    else:
        break

print(cnt)
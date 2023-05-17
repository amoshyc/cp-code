N = int(input())
A = list([int(x) for x in input().split()])
L = int(10**18)

if 0 in A:
    ans = 0
else:
    ans = 1
    for a in A:
        ans = ans * a
        if ans > L:
            ans = -1
            break

print(ans)
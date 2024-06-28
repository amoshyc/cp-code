inp = input().split()
L, R, K = int(inp[0]), int(inp[1]), int(inp[2])

if K is 1:
    print('{}'.format(1 if L is 1 else -1))
else:
    ans = []
    i = 1
    while i <= R:
        if i >= L:
            ans.append(i)
        i *= K

    if len(ans) is 0:
        print('-1')
    else:
        print(' '.join(map(str, ans)))

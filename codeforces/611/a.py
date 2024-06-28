inp = input().split()

num = int(inp[0])



if inp[-1] == 'week':
    print('53' if num in [5, 6] else '52')
else:
    if num == 29:
        print('12')
    elif num == 30:
        print('11')
    elif num == 31:
        print('7')
    else:
        print('12')

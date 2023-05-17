from collections import deque

S = input()

heads = deque([])
tails = deque([])
reverse = False

Q = int(input())
for _ in range(Q):
    cmds = input().split()
    if cmds[0] == '1':
        reverse = not reverse
        heads, tails = tails, heads
    else:
        f, c = cmds[1], cmds[2]
        if f == '1':
            if reverse:
                heads.append(c)
            else:
                heads.appendleft(c)
        else:
            if reverse:
                tails.appendleft(c)
            else:
                tails.append(c)
    
    # if reverse:
    #     print(list(heads)[::-1], S[::-1], list(tails)[::-1])
    # else:
    #     print(list(heads), S, list(tails))
    # print('-' * 10)

if reverse:
    heads = list(heads)[::-1]
    tails = list(tails)[::-1]
    S = S[::-1]

heads = ''.join(map(str, heads))
tails = ''.join(map(str, tails))
ans = ''.join([heads, S, tails])
print(ans)
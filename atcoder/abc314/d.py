N = int(input())
S = input()
Q = int(input())

cmds = []
for i in range(N):
    cmds.append((1, i, S[i]))
for _ in range(Q):
    op, p, x = input().split()
    cmds.append((int(op), int(p) - 1, x))

ans = [' ' for _ in range(N)]
last = None
for op, p, x in reversed(cmds):
    if op == 1 and ans[p] == ' ':
        if last is None:
            ans[p] = x
        elif last == 'u':
            ans[p] = x.upper()
        else:
            ans[p] = x.lower()
    
    if op == 2 and last is None:
        last = 'l'
    if op == 3 and last is None:
        last = 'u'

print(''.join(ans))
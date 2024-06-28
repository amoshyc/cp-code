# ???????caba
# aba????caba
# abac???caba

TC = int(input())

def find(S):
    idxs = []
    for i in range(len(S) - 7 + 1):
        if S[i:i+7] == 'abacaba':
            idxs.append(i)
    return idxs

def solve():
    N = int(input())
    S = input()
    T = 'abacaba'

    idxs = find(S)
    if len(idxs) >= 2:
        return False, ''
    if len(idxs) == 1:
        return True, S.replace('?', 'z')
    
    for i in range(N - 7 + 1):
        if all(S[i + j] == T[j] or S[i + j] == '?' for j in range(7)):
            res = (S[:i] + T + S[i + 7:]).replace('?', 'z')
            if len(find(res)) == 1:
                return True, res

    return False, ''
    

for _ in range(TC):
    ok, s = solve()
    if ok:
        print('Yes')
        print(s)
    else:
        print('No')

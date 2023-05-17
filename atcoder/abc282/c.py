N = int(input())
S = list(input())

f = 0
for i, c in enumerate(S):
    if c == '"':
        f ^= 1
    
    if c == ',':
        if f == 0:
            S[i] = '.'

print(''.join(S))
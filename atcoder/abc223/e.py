
def ceil_div(x, y):
    return (x + y - 1) // y

def check(x, y, a, b, c):
    # Case 1
    ha = ceil_div(a, x)
    hb = ceil_div(b, x)
    hc = ceil_div(c, x)
    if ha + hb + hc <= y:
        return True
    
    # Case 2
    ha = ceil_div(a, x)
    hbc = y - ha
    if hbc > 0:
        xb = ceil_div(b, hbc)
        xc = ceil_div(c, hbc)
        if xb + xc <= x:
            return True
    
    return False

from itertools import permutations

X, Y, A, B, C = map(int, input().split())
ans = False
for (x, y) in permutations([X, Y]):
    for (a, b, c) in permutations([A, B, C]):
        if check(x, y, a, b, c):
            ans = True

print('Yes' if ans else 'No')
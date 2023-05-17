from math import sqrt

def solve():
    S, P = map(int, input().split())
    M1 = (S + int(sqrt(S ** 2 - 4 * P))) // 2
    if M1 > 0:
        N1 = P // M1
        if M1 + N1 == S and N1 > 0:
            return True
    
    M2 = (S - int(sqrt(S ** 2 - 4 * P))) // 2
    if M2 > 0:
        N2 = P // M2
        if M2 + N2 == S and N2 > 0:
            return True

    return False

print('Yes' if solve() else 'No')
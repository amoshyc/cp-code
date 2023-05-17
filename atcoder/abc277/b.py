def solve():
    N = int(input())
    S = [input() for _ in range(N)]

    for s in S:
        if s[0] not in 'HDCS':
            return False
        if s[1] not in 'A23456789TJQK':
            return False
    
    if len(set(S)) != N:
        return False
    
    return True

print('Yes' if solve() else 'No')
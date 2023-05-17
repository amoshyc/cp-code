N = int(input())
C = input()

cntR = C.count('R')
cntW = C.count('W')
if cntR == 0 or cntW == 0:
    print(0)
else:
    T = 'R' * cntR + 'W' * cntW
    ans = sum([1 for a, b in zip(C, T) if a != b]) // 2
    print(ans)
    

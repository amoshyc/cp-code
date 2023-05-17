A, B, C, D = map(int, input().split())

if C <= A <= D or C <= B <= D or A <= C <= B or A <= D <= B:
    print('Yes')
else:
    print('No')
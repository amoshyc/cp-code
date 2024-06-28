def solve():
    r, g, b, w = map(int, input().split())

    if r % 2 + g % 2 + b % 2 + w % 2 <= 1:
        return 'Yes'
    
    if min(r, g, b) >= 1:
        r, g, b, w = r - 1, g - 1, b - 1, w + 3
        if r % 2 + g % 2 + b % 2 + w % 2 <= 1:
            return 'Yes'
    
    return 'No'

TC = int(input())
for tc in range(TC):
    print(solve())
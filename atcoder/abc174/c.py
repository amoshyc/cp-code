K = int(input())

def solve():
    if 7 % K == 0:
        return 1

    val = 7
    for i in range(2, K + 1):
        val = val * 10 + 7
        val = val % K
        if val == 0:
            return i
    
    return -1

print(solve())

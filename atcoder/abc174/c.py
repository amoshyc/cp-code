K = int(input())

def solve():
    val = 0
    for i in range(1, K + 1):
        val = val * 10 + 7
        val = val % K
        if val == 0:
            return i
    
    return -1

print(solve())
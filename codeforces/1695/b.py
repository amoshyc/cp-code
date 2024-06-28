def solve():
    N = int(input())
    A = [int(x) for x in input().split()]
    
    if N % 2 == 0:
        idx1, idx2 = -1, -1
        val1, val2 = 10 ** 10, 10 ** 10
        for i, a in enumerate(A):
            if i % 2 == 0:
                if a < val1:
                    val1 = a
                    idx1 = i
            else:
                if a < val2:
                    val2 = a
                    idx2 = i
        
        if val1 < val2:
            return 'Joe'
        if val1 > val2:
            return 'Mike'
        return 'Mike' if idx1 > idx2 else 'Joe'
    else:
        return 'Mike'


TC = int(input())
for _ in range(TC):
    print(solve())
def solve():
    N = int(input())
    A = [int(x) for x in input().split()]
    
    prev2 = None
    prev3 = None
    ans = []
    ones = []
    next_row = N - 1

    for col in range(N - 1, -1, -1):
        if A[col] == 0:
            continue
        if A[col] == 1:
            ans.append((next_row, col))
            ones.append((next_row, col))
            next_row -= 1
        if A[col] == 2:
            if len(ones) == 0:
                return -1
            r, c = ones.pop()
            ans.append((r, col))
            prev2 = (r, col)
        if A[col] == 3:
            if prev3 is not None:
                r, c = prev3
            elif prev2 is not None:
                r, c = prev2
                prev2 = None
            elif len(ones) == 0:
                return -1
            else:
                r, c = ones.pop()
            ans.append((next_row, c))
            ans.append((next_row, col))
            prev3 = (next_row, col)
            next_row -= 1

    return ans
    
ans = solve()
if ans == -1:
    print(-1)
else:
    print(len(ans))
    for r, c in ans:
        print(r + 1, c + 1)
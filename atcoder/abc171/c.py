N = int(input()) - 1

lb, ub = 0, 0

for n_digit in range(1, 20):
    lb = ub
    ub = (ub + 1) * 26
    # print(n_digit, lb, ub)

    if lb <= N < ub:
        off = N - lb
        ans = ['a' for _ in range(n_digit)]
        idx = 1
        while off > 0:
            ans[-idx] = chr(ord('a') + off % 26)
            off //= 26
            idx += 1
        
        print(''.join(ans))
        break
def solve():
    S = list(input())
    T = list(input())
    if S == T:
        return 'Yes'

    for i in range(1, len(S)):
        S[i - 1], S[i] = S[i], S[i - 1]
        if S == T:
            return 'Yes'
        S[i - 1], S[i] = S[i], S[i - 1]

    return 'No'

print(solve())
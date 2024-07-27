def next_permutation_(arr):
    n = len(arr)
    k = next(i for i in reversed(range(n - 1)) if arr[i] < arr[i + 1])
    j = next(i for i in reversed(range(n)) if arr[i] > arr[k])
    arr[k], arr[j] = arr[j], arr[k]
    arr[(k + 1) :] = arr[(k + 1) :][::-1]


N, K = map(int, input().split())
S = list(input())
S.sort()

ans = 0
while True:
    has_palindrome = False
    for i in range(N - K + 1):
        if all(S[i + j] == S[i + K - 1 - j] for j in range(K)):
            has_palindrome = True
            break
    if not has_palindrome:
        ans += 1

    try:
        next_permutation_(S)
    except StopIteration:
        break

print(ans)

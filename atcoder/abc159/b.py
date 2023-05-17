def is_palindrome(s):
    return s == s[::-1]

s = input()

if len(s) % 2 == 0:
    p = len(s) // 2
    if is_palindrome(s) and is_palindrome(s[:p]) and is_palindrome(s[p:]):
        print('Yes')
    else:
        print('No')
else:
    p = len(s) // 2
    if is_palindrome(s) and is_palindrome(s[:p]) and is_palindrome(s[p + 1:]):
        print('Yes')
    else:
        print('No')
N = int(input())
S = input()

n_remove = 0
stack = []
for c in S:
    stack.append(c)
    if len(stack) >= 3 and stack[-3] == 'f' and stack[-2] == 'o' and stack[-1] == 'x':
        stack.pop()
        stack.pop()
        stack.pop()
        n_remove += 3

print(len(S) - n_remove)

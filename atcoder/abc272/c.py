N = int(input())
A = [int(x) for x in input().split()]
A.sort()

evens = []
odds = []
for a in A:
    if a % 2 == 0:
        evens.append(a)
    else:
        odds.append(a)

ans = -1
if len(evens) >= 2:
    ans = max(ans, evens[-1] + evens[-2])
if len(odds) >= 2:
    ans = max(ans, odds[-1] + odds[-2])
print(ans)
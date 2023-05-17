S = input()
check1 = sum(1 for c in S if c.isupper()) > 0
check2 = sum(1 for c in S if c.islower()) > 0
check3 = len(set(S)) == len(S)
print('Yes' if all([check1, check2, check3]) else 'No')
S = input()
if S == 'RRR':
    print(3)
elif S.find('RR') != -1:
    print(2)
elif S.find('R') != -1:
    print(1)
else:
    print(0)
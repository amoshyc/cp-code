N = int(input())
A = [input() for _ in range(N)]

print('AC x {}'.format(len([x for x in A if x == 'AC'])))
print('WA x {}'.format(len([x for x in A if x == 'WA'])))
print('TLE x {}'.format(len([x for x in A if x == 'TLE'])))
print('RE x {}'.format(len([x for x in A if x == 'RE'])))
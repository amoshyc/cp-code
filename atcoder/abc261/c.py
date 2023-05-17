from collections import defaultdict

N = int(input())

cnt = defaultdict(int)

for _ in range(N):
    x = input()
    c = cnt[x]

    if c == 0:
        print(x)
    else:
        print(f'{x}({c})')

    cnt[x] += 1

    

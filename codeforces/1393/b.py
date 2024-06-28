from collections import defaultdict

N = int(input())
A = input().split()

cnt = defaultdict(int)
for x in A:
    cnt[x] += 1

has_8 = set() # 8 or more
has_6 = set() # 6 or 7
has_4 = set() # 4 or 5
has_2 = set() # 2 or 3
for k, v in cnt.items():
    if v >= 8:
        has_8.add(k)
        continue
    if v >= 6:
        has_6.add(k)
        continue
    if v >= 4:
        has_4.add(k)
        continue
    if v >= 2:
        has_2.add(k)
        continue

Q = int(input())
for _ in range(Q):

    cmd, x = input().split()
    if cmd == '+':
        if cnt[x] == 7:
            has_8.add(x)
            has_6.remove(x)
        if cnt[x] == 5:
            has_6.add(x)
            has_4.remove(x)
        if cnt[x] == 3:
            has_4.add(x)
            has_2.remove(x)
        if cnt[x] == 1:
            has_2.add(x)
        cnt[x] += 1
    else:
        if cnt[x] == 2:
            has_2.remove(x)
        if cnt[x] == 4:
            has_4.remove(x)
            has_2.add(x)
        if cnt[x] == 6:
            has_6.remove(x)
            has_4.add(x)
        if cnt[x] == 8:
            has_8.remove(x)
            has_6.add(x)
        cnt[x] -= 1
    
    # print('-' * 10)
    # print('8:', has_8)
    # print('6:', has_6)
    # print('4:', has_4)
    # print('2:', has_2)
    # print(cnt)

    cond0 = len(has_8) >= 1
    cond1 = len(has_6) >= 2
    cond2 = len(has_6) == 1 and (len(has_4) >= 1 or len(has_2) >= 1)
    cond3 = len(has_4) >= 2
    cond4 = len(has_4) == 1 and len(has_2) >= 2
    
    if cond0 or cond1 or cond2 or cond3 or cond4:
        print('YES')
    else:
        print('NO')
        

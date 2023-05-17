N = int(input())
A, B = [], []
for _ in range(N):
    a, b = map(int, input().split())
    A.append(a)
    B.append(b)
S = sum(A)

# check(pos) = time from left > time from right
# 0 0 0 0 1 1 1
def check(pos):
    pref = 0
    timeL = 0
    for a, b in zip(A, B):
        if pref <= pos <= pref + a:
            d = pos - pref
        else:
            d = a
        timeL += d / b
        pref += d
        
    suff = 0
    timeR = 0
    for a, b in zip(reversed(A), reversed(B)):
        if S - (suff + a) <= pos <= (S - suff):
            d = S - suff - pos
        else:
            d = a
        timeR += d / b
        suff += d
    
    # print(pos, ':', timeL, timeR)
    return timeL > timeR

# print(check(2))
# print(check(3))
# print(check(4))

lb, ub = 0, S
for _ in range(100):
    m = (lb + ub) / 2
    if check(m):
        ub = m
    else:
        lb = m
print(lb)
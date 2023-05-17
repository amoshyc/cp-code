def check(h, m):
    A, B = divmod(h, 10)
    C, D = divmod(m, 10)
    hour = A * 10 + C
    minute = B * 10 + D
    return 0 <= hour <= 23 and 0 <= minute <= 59

H, M = map(int, input().split())
while True:
    if check(H, M) is True:
        print(H, M)
        break

    M += 1
    if M == 60:
        H += 1
        M = 0
        if H == 24:
            H = 0
        
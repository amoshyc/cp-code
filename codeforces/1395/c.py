N, M = map(int, input().split())
A = list(map(int, input().split()))
B = set(map(int, input().split()))

def check(ans):
    for a in A:
        all_c = [a & b for b in range(1 << 9) if b in B]
        if not any((c | ans) == ans for c in all_c):
            return False
    return True

for ans in range(0, (1 << 9)):
    if check(ans):
        print(ans)
        break
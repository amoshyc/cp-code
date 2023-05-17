A, B, C = map(int, input().split())
K = int(input())

cnt = 0
while B <= A:
    cnt += 1
    B = B * 2

while C <= B:
    cnt += 1
    C = C * 2

print('Yes' if cnt <= K else 'No')
n = int(input())
s = input()

keys = []
for i in range(100):
    keys.append('ogo' + ('go' * i));

for k in keys[::-1]:
    s = s.replace(k, '***')

print(s)

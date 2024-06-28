time = input()
a = int(input())

h = int(time[:2])
m = int(time[3:])

a = a % 1440

dh = a // 60
dm = a % 60

m += dm
carry = m // 60
m %= 60

h = (h + dh + carry) % 24

print('{:02}:{:02}'.format(h, m))

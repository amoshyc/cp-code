N = int(input()) % 10
print('hon' if N in [2, 4, 5, 7, 9] else 'pon' if N in [0, 1, 6, 8] else 'bon')
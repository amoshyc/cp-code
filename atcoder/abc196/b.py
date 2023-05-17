X = input()
idx = X.find('.')
print(X[:idx] if idx != -1 else X)
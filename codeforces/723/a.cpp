#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
	ios::sync_with_stdio(false);
	cin.tie(0);

	int a, b, c;
	cin >> a >> b >> c;

	int s = a + b + c;
	int mx = max(a, max(b, c));
	int mn = min(a, min(b, c));
	int m = s - mx - mn;

	cout << abs(m - mx) + abs(m - mn) << endl;

	return 0;
}

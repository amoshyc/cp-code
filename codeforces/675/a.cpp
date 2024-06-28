#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ll a, b, c;
    cin >> a >> b >> c;

    if (c == 0) {
        cout << ((a == b) ? "YES" : "NO") << endl;
    }
    else {
        ll r = (b - a) % c;
        ll k = (b - a) / c;
        if (r == 0 && k >= 0) {
            cout << "YES\n";
        }
        else {
            cout << "NO\n";
        }
    }

    return 0;
}

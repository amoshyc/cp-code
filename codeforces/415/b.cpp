#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

ll ceildiv(ll a, ll b) {
    return a / b + (a % b != 0);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    cout.tie(0);

    int N; ll a, b;
    cin >> N >> a >> b;

    for (int i = 0; i < N; i++) {
        ll x; cin >> x;
        ll money = x * a / b; // floor
        ll rest = x - ceildiv(money * b, a);
        cout << rest << " ";
    }
    cout << endl;

    return 0;
}

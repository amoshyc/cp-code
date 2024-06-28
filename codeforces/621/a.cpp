#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const ll inf = 1e9 + 1ll;

int main() {
    int n; cin >> n;
    ll min_odd = inf;
    ll sum = 0;
    for (int i = 0; i < n; i++) {
        ll inp; cin >> inp;
        sum += inp;

        if (inp % 2 == 1)
            min_odd = min(min_odd, inp);
    }

    if (sum % 2 == 0)
        cout << sum << "\n";
    else {
        cout << sum - min_odd << "\n";
    }

    return 0;
}

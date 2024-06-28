#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

ll money, a, b, c;
ll cnt = 0;

int main() {
    cin >> money >> a >> b >> c;

    if (money < b) {
        cout << money / a << "\n";
    }
    else { // money >= b
        if (a < b - c) cout << money / a << "\n";
        else {
            ll cnt = (money - c) / (b - c);
            ll rest = money - cnt * (b - c);
            cnt += rest / a;

            cout << cnt << "\n";
        }
    }

    return 0;
}

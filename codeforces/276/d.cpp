#include <bits/stdc++.h>
using namespace std;

typedef long long ull;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    cout.tie(0);

    // http://cs.stackexchange.com/a/29509

    ull l, r;
    cin >> l >> r;

    if (l == r) {
        cout << "0\n";
        return 0;
    }

    int pos = 0; // pos of first different
    for (int i = 63; i >= 0; i--) {
        if ((l & (1ll << i)) != (r & (1ll << i))) {
            pos = i;
            break;
        }
    }

    cout << ((1ll << (pos + 1)) - 1) << endl;

    return 0;
}

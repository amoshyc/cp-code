#include <vector>
#include <iostream>
#include <algorithm>
using namespace std;

using ll = long long;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    ll p, q, r;
    cin >> N >> p >> q >> r;

    auto arr = vector<ll>(N, 0);
    for (int i = 0; i < N; i++) {
        cin >> arr[i];
    }

    auto pref = vector<ll>(N, 0);
    pref[0] = arr[0];
    for (int i = 1; i < N; i++) {
        pref[i] = pref[i - 1] + arr[i];
    }

    for (int x = 0; x < N; x++) {
        ll sub = ((x == 0) ? 0 : pref[x - 1]);
        ll y = upper_bound(pref.begin(), pref.end(), sub + p) - pref.begin();
        ll z = upper_bound(pref.begin(), pref.end(), sub + p + q) - pref.begin();
        ll w = upper_bound(pref.begin(), pref.end(), sub + p + q + r) - pref.begin();

        if (y == 0 || z == 0 || w == 0) {
            continue;
        }

        bool ok1 = pref[y - 1] - sub == p;
        bool ok2 = pref[z - 1] - sub == p + q;
        bool ok3 = pref[w - 1] - sub == p + q + r;

        if (x < y && y < z && z < w && w <= N && ok1 && ok2 && ok3) {
            cout << "Yes" << endl;
            return 0;
        }
    }

    cout << "No" << endl;

    return 0;
}
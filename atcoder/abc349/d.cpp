#include <iostream>
#include <vector>

using namespace std;
using i64 = long long;

template <typename T> int sz(const T &c) { return c.size(); }

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    i64 l, r;
    cin >> l >> r;

    vector<pair<i64, i64>> ans;
    i64 idx = l;
    while (idx < r) {
        for (int i = 60; i >= 0; i--) {
            i64 l = (1ll << i);
            if (idx + l <= r && idx % l == 0) {
                auto j = idx / l;
                idx += l;
                ans.push_back({l * j, l * (j + 1)});
                break;
            }
        }
    }

    cout << sz(ans) << endl;
    for (auto [a, b] : ans) {
        cout << a << " " << b << "\n";
    }

    return 0;
}
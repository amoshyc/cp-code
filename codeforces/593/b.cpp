#include <bits/stdc++.h>
using namespace std;

#define st first
#define nd second

typedef long long ll;
typedef pair<ll, ll> pll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    cin >> N;

    ll x1, x2;
    cin >> x1 >> x2;

    vector<pll> v;
    for (int i = 0; i < N; i++) {
        ll k, b;
        cin >> k >> b;
        ll y1 = k * x1 + b;
        ll y2 = k * x2 + b;
        v.push_back(pll(y1, y2));
    }

    sort(v.begin(), v.end()); // sort by left y，由小到大
    for (int i = 1; i < int(v.size()); i++) {
        ll ry1 = v[i].nd;
        ll ry2 = v[i - 1].nd;
        if (ry1 < ry2) { // 右端錯位
            cout << "YES" << endl;
            return 0;
        }
    }

    cout << "NO" << endl;

    return 0;
}

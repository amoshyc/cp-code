#include <iostream>
using namespace std;


using ll = long long;
pair<ll, ll> euclid_div(ll a, ll b) {
    ll q = a / b;
    ll r = a % b;
    if (r < 0) {
        r += b;
        q -= 1;
    }
    return {q, r};
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    ll x;
    cin >> x;
    auto [q, _] = euclid_div(x, 10);
    cout << q << endl;

    return 0;
}
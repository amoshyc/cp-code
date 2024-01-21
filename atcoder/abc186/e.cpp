#include <algorithm>
#include <iostream>
#include <tuple>
#include <vector>
using namespace std;
using ll = long long;

tuple<ll, ll, ll> extgcd(ll a, ll b) {
    if (b == 0) {
        return {1, 0, a};
    } else {
        auto [x1, y1, g] = extgcd(b, (a % b + b) % b);
        return {y1, x1 - a / b * y1, g};
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int TC;
    cin >> TC;
    while (TC--) {
        int N, S, K;
        cin >> N >> S >> K;

        // Find minimum x (x >= 0) such that
        // S + xK = 0 (mod N)
        // Kx + (-N)y = -S
        // <->
        // ax + by = m, has solution
        //   x = m/g x0 + k b/g = p x0 + k q where k is int, g = gcd(a, b)
        //   y = m/g y0 + k a/g = p x0 + k r where k is int, g = gcd(a, b)
        // if g divides m
        ll a = K;
        ll b = -N;
        ll m = -S;
        auto [x0, y0, g] = extgcd(a, b);
        if (m % g != 0) {
            cout << -1 << endl;
        } else {
            ll p = m / g, q = b / g;
            ll k = 0;
            if ((p * x0) < 0) {
                k = (abs(p * x0) + (q - 1)) / q;
            } else {
                k = -(abs(p * x0) / q);
            }
            ll x = p * x0 + k * q;
            cout << x << endl;
        }
    }

    return 0;
}
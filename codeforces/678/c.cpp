#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

ll gcd(ll a, ll b) {
    while (b) {
        // a, b = b, a % b
        ll temp = a % b;
        a = b;
        b = temp;
    }
    return a;
}

ll lcm(ll a, ll b) {
    return a / gcd(a, b) * b;
}

int main() {
    ll N, A, B, P, Q;
    cin >> N >> A >> B >> P >> Q;

    ll cnt_red_or_blue = N / lcm(A, B);
    ll cnt_red = N / A - cnt_red_or_blue;
    ll cnt_blue = N / B - cnt_red_or_blue;

    ll res1 = cnt_red * P + cnt_blue * Q + cnt_red_or_blue * P;
    ll res2 = cnt_red * P + cnt_blue * Q + cnt_red_or_blue * Q;
    cout << max(res1, res2) << endl;

    return 0;
}

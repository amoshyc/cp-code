#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    ll a, b;
    cin >> a >> b;

    int fac_a[3] = {0};
    int fac_b[3] = {0};
    // a
    while (a % 2 == 0) {
        fac_a[0]++;
        a /= 2;
    }
    while (a % 3 == 0) {
        fac_a[1]++;
        a /= 3;
    }
    while (a % 5 == 0) {
        fac_a[2]++;
        a /= 5;
    }

    // b
    while (b % 2 == 0) {
        fac_b[0]++;
        b /= 2;
    }
    while (b % 3 == 0) {
        fac_b[1]++;
        b /= 3;
    }
    while (b % 5 == 0) {
        fac_b[2]++;
        b /= 5;
    }

    if (a != b) {
        cout << "-1" << endl;
        return 0;
    }

    int cnt = 0;
    for (int i = 0; i < 3; i++) {
        cnt += abs(fac_a[i] - fac_b[i]);
    }

    cout << cnt << endl;

    return 0;
}

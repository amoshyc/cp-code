#include <iostream>
#include <utility>
using namespace std;

using i64 = long long;

pair<i64, i64> euclidean_div(i64 a, i64 b) {
    if (b < 0) {
        a = -a;
        b = -b;
    }
    i64 q = a / b;
    if (q < 0 && a % b != 0) {
        q -= 1;
    }
    return {q, a - b * q};
}

i64 floor_div(i64 a, i64 b) {
    auto [q, r] = euclidean_div(a, b);
    return q;
}

i64 ceil_div(i64 a, i64 b) {
    auto [q, r] = euclidean_div(a, b);
    return q + ((r != 0) ? 1 : 0);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    i64 a, m, l, r;
    cin >> a >> m >> l >> r;

    i64 lb = ceil_div(l - a, m);
    i64 ub = floor_div(r - a, m);
    i64 ans = max(ub - lb + 1, 0ll);
    cout << ans << endl;

    return 0;
}
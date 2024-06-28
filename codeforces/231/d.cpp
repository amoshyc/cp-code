#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int a[6];
    int x, y, z;
    int sx, sy, sz;

    cin >> x >> y >> z;
    cin >> sx >> sy >> sz;
    for (int i = 0; i < 6; i++)
        cin >> a[i];

    int ans = 0;
    if (y < 0) ans += a[0];
    if (y > sy) ans += a[1];
    if (z < 0) ans += a[2];
    if (z > sz) ans += a[3];
    if (x < 0) ans += a[4];
    if (x > sx) ans += a[5];

    cout << ans << endl;

    return 0;
}

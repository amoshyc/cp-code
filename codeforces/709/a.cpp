#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int n, b, d;
    cin >> n >> b >> d;

    int ans = 0;
    int cnt = 0;

    for (int i = 0; i < n; i++) {
        int a; cin >> a;
        if (a <= b) {
            cnt += a;
            if (cnt > d) {
                ans++;
                cnt = 0;
            }
        }
    }

    cout << ans << endl;

    return 0;
}

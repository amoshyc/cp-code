#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, H;
    cin >> N >> H;

    int ans = 0;
    for (int i = 0; i < N; i++) {
        int h; cin >> h;
        ans += ((h > H) ? 2 : 1);
    }

    cout << ans << endl;

    return 0;
}

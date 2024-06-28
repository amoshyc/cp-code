#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    int N; ll M;
    cin >> N >> M;

    int res[50];
    int s = 0, t = N - 1;
    int idx = 1;

    for (int len = N; len >= 1; len--) {
        ll cnt = (1ll << (len - 2));
        if (M <= cnt) {
            res[s++] = idx++;
        }
        else {
            res[t--] = idx++;
            M = M - cnt;
        }
    }

    for (int i = 0; i < N; i++)
        cout << res[i] << " ";
    cout << endl;

    return 0;
}

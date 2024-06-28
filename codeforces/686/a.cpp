#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N, X;
    cin >> N >> X;
    string s; getline(cin, s);

    int cnt = 0;
    ll n = X;

    for (int i = 0; i < N; i++) {
        string flag; ll d; cin >> flag >> d;

        if (flag[0] == '+') {
            n += d;
        }
        else {
            if (n >= d) {
                n -= d;
            }
            else {
                cnt++;
            }
        }
    }

    cout << n << " " << cnt << "\n";

    return 0;
}

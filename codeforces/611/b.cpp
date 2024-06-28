#include <bits/stdc++.h>
using namespace std;

typedef unsigned long long ull;

ull length(ull a) {
    int n = 0;
    while (a) {
        n++;
        a >>= 1;
    }
    return n;
}

int main() {
    ull a, b;
    cin >> a >> b;

    int n = length(b) + 1;
    // cout << n << endl;

    int cnt = 0;
    for (int len = 1; len <= n; len++) {
        ull c = (1ULL << (len + 1ULL)) - 1ULL;
        // cout << "c " << c << endl;

        for (int i = 0; i < len; i++) {
            ull d = c ^ (1ULL << i);
            if (a <= d && d <= b) {
                cnt++;
                // cout << ": " << d << endl;
            }
        }
    }

    cout << cnt << endl;



    return 0;
}

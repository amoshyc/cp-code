#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

inline bool C(int x, int* p) {
    return x % p[0] % p[1] % p[2] % p[3] == x;
}

int get_cnt(int x, int* p) {
    int cnt = 0;
    do {
        if (C(x, p))
            cnt++;
    } while (next_permutation(p, p + 4));

    return cnt;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int p[4]; int a, b;
    for (int i = 0; i < 4; i++) cin >> p[i];
    cin >> a >> b;

    sort(p, p + 4);

    int res = 0;
    for (int i = a; i <= b; i++) {
        if (get_cnt(i, p) >= 7) {
            res++;
        }
    }

    cout << res << endl;

    return 0;
}

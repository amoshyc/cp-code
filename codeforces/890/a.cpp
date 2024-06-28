#include <bits/stdc++.h>
using namespace std;

#define sz(x) (int(x.size()))
typedef long long ll;
inline int getint() { int inp; scanf(" %d", &inp); return inp; }
inline int getll() { ll inp; scanf(" %lld", &inp); return inp; }

int a[6];

bool solve() {
    int f[6] = {0, 1, 2, 3, 4, 5};
    do {
        if (a[f[0]] + a[f[1]] + a[f[2]] == a[f[3]] + a[f[4]] + a[f[5]]) {
            return true;
        }
    } while (next_permutation(f, f + 6));
    return false;
}

int main() {
    for (int i = 0 ; i < 6; i++) {
        a[i] = getint();
    }

    puts((solve() ? "YES": "NO"));

    return 0;
}
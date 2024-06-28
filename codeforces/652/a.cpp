#include <bits/stdc++.h>
using namespace std;

int h1, h2; int a, b;

int solve() {
    int cnt = 0;

    if (a == b) {
        if (h1 + 8 * a >= h2)
            return 0;
        return -1;
    }

    while (true) {
        // day: 8 hour
        h1 += 8 * a;
        if (h1 >= h2)
            return cnt;

        cnt++;

        // night: 12 hour
        h1 -= 12 * b;
        if (h1 <= 0 && b >= a)
            return -1;

        // day: 4 hour
        h1 += 4 * a;
        if (h1 >= h2)
            return cnt;
    }

    return cnt;
}

int main() {
    cin >> h1 >> h2 >> a >> b;
    cout << solve() << endl;

    return 0;
}

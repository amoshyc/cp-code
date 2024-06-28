#include <bits/stdc++.h>
using namespace std;

#define x first
#define y second

typedef pair<int, int> pii;
set<pii> s;

int gcd(int a, int b) {
    while (b) {
        int temp = a % b;
        a = b;
        b = temp;
    }
    return a;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    cin >> N;

    int x0, y0;
    cin >> x0 >> y0;

    for (int i = 0; i < N; i++) {
        int x, y;
        cin >> x >> y;
        pii v(x - x0, y - y0);
        if (v.y < 0) {
            v.x = -v.x;
            v.y = -v.y;
        }

        int g = gcd(v.x, v.y);
        v.x /= g;
        v.y /= g;

        s.insert(v);
    }

    cout << s.size() << endl;

    return 0;
}

#include <bits/stdc++.h>
using namespace std;

// const double pi = atan2(0, -1);
const double eps = 1e-8;

bool lt(double a, double b) { return a - b < -eps; }
// bool gt(double a, double b) { return a - b > +eps; }
// bool eq(double a, double b) { return fabs(a - b) < eps; }

bool inside(double x, double y, double x1, double y1, double x2, double y2) {
    // cout << x << " " << y << ": ";
    // cout << x1 << " " << y1 << " " << x2 << " " << y2 << endl;
    // return (x1 < x && x < x2) && (y2 < y && y < y1);
    return (lt(x1, x) && lt(x, x2) && lt(y2, y) && lt(y, y1));
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    double A, x, y;
    cin >> A >> x >> y;

    // enum the "level" and check if (x, y) is in square(s)

    // level 0
    if (inside(x, y, -A / 2.0, A, +A / 2.0, 0.0)) {
        cout << "1" << endl;
        return 0;
    }

    // other levels
    int id = 2;
    for (int level = 1; ; level++) {
        if (level & 1) {
            if (inside(x, y, -A / 2.0, A * level + A, +A / 2.0, A * level)) {
                cout << id << endl;
                return 0;
            }
            id++;
        }
        else {
            // left one
            if (inside(x, y, -A, A * level + A, 0.0, A * level)) {
                cout << id << endl;
                return 0;
            }
            id++;
            // right one
            if (inside(x, y, 0.0, A * level + A, +A, A * level)) {
                cout << id << endl;
                return 0;
            }
            id++;
        }

        if (A * level + A > y)
            break;
    }

    cout << "-1" << endl;

    return 0;
}

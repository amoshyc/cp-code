#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const double eps = 1e-8;

double X1, Y1, R1;
double X2, Y2, R2;

inline bool lt(double a, double b) { return a - b < -eps; }
inline bool gt(double a, double b) { return a - b > +eps; }
inline bool eq(double a, double b) { return fabs(a - b) < eps; }

double dis(double a, double b, double c, double d) {
    double dx = a - c;
    double dy = b - d;
    return sqrt(dx * dx + dy * dy);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> X1 >> Y1 >> R1;
    cin >> X2 >> Y2 >> R2;

    if (R2 < R1) {
        swap(X1, X2);
        swap(Y1, Y2);
        swap(R1, R2);
    }

    // R1 is less
    double d = dis(X1, Y1, X2, Y2);

    if (lt(d + R1, R2) || eq(d + R1, R2)) { // 包含
        cout << fixed << setprecision(10) << (R2 - d - R1) / 2 << endl;
        return 0;
    }

    if (lt(d, R1 + R2) || eq(d, R1 + R2)) { // 相交
        cout << "0" << endl;
        return 0;
    }

    // 無相交
    cout << fixed << setprecision(10) << (d - R1 - R2) / 2 << endl;
    return 0;
}

#include <bits/stdc++.h>
using namespace std;

const double pi = atan2(0, -1);
// const double eps = 1e-15;

long long d2(long long x1, long long y1, long long x2, long long y2) {
    return (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
}

int main() {
    long long x1, y1, r1;
    long long x2, y2, r2;
    // scanf("%lld %lld %lld", &x1, &y1, &r1);
    // scanf("%lld %lld %lld", &x2, &y2, &r2);
    cin >> x1 >> y1 >> r1;
    cin >> x2 >> y2 >> r2;

    long long r = min(r1, r2), R = max(r1, r2);
    long long dis2 = d2(x1, y1, x2, y2);

    if (dis2 >= (r1 + r2) * (r1 + r2)) { // d >= r1 + r2
        // puts("0");
        cout << "0\n";
        return 0;
    }

    if (dis2 <= (R - r) * (R - r)) { // d <= R - r
        long double area = pi * r * r;
        // printf("%.10Lf\n", area);
        cout << fixed << setprecision(10) << area << "\n";
        return 0;
    }

    long double d_d = sqrt((long double) dis2);

    long double d_cos1 = (r * r + d_d * d_d - R * R) / (2.0 * r * d_d);
    long double theta1 = 2 * acos(d_cos1);
    long double d_cos2 = (R * R + d_d * d_d - r * r) / (2.0 * R * d_d);
    long double theta2 = 2 * acos(d_cos2);

    // printf("%lf, %lf\n", d_cos1, d_cos2);

    long double area1 = r * r * (theta1 - sin(theta1)) / 2.0;
    long double area2 = R * R * (theta2 - sin(theta2)) / 2.0;
    // printf("%.10Lf\n", area1 + area2);
    cout << fixed << setprecision(10) << area1 + area2 << "\n";

    return 0;
}

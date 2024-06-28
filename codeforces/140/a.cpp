#include <bits/stdc++.h>
using namespace std;

const double pi = atan2(0, -1);
const double eps = 1e-8;
int N, R, r;

int main() {
    scanf("%d %d %d", &N, &R, &r);

    if (r > R) {
        puts("NO");
        return 0;
    }
    if (r * 2 > R) {
        puts((N <= 1) ? "YES" : "NO");
        return 0;
    }
    if (r * 2 == R) {
        puts((N <= 2) ? "YES" : "NO");
        return 0;
    }

    double theta = 2.0 * asin(double(r) / double(R - r));
    double cnt = 2.0 * pi / theta;
    puts((cnt - double(N) >= -eps) ? "YES" : "NO"); // cnt >= N

    return 0;
}
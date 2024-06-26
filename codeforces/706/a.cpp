#include <bits/stdc++.h>
using namespace std;

int N;
double A, B;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    double ans = 1e10;
    cin >> A >> B;
    cin >> N;
    for (int i = 0; i < N; i++) {
        double x, y, v;
        cin >> x >> y >> v;
        double dx = x - A;
        double dy = y - B;
        double dis = sqrt(dx * dx + dy * dy);
        double t = dis / v;

        ans = min(ans, t);
    }

    cout << fixed << setprecision(10) << ans << endl;

    return 0;
}

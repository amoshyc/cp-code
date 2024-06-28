#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const double eps = 1e-8;

const int MAX_N = 100;
int N, vb, vs, xu, yu;
int stop[MAX_N];

double d(double x1, double y1, double x2, double y2) {
    double dx = x1 - x2;
    double dy = y1 - y2;
    return sqrt(double(dx * dx + dy * dy));
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> N >> vb >> vs;
    for (int i = 0; i < N; i++)
        cin >> stop[i];
    cin >> xu >> yu;

    double min_time = 1e18;
    int min_time_idx = -1;
    double time_to_stop = 0.0;
    for (int i = 1; i < N; i++) {
        time_to_stop = double(stop[i] - 0.0) / vb;
        double time_total = time_to_stop + d(stop[i], 0, xu, yu) / vs;

        // cout << i << ": ";
        // cout << time_to_stop << " " << time_total << endl;

        if (time_total - min_time <= +eps) { // time_total <= min_time
            min_time = time_total;
            min_time_idx = i + 1;
        }
    }

    cout << min_time_idx << endl;

    return 0;
}

#include <bits/stdc++.h>
using namespace std;

struct Pt {
    int x, y;
};

double d(const Pt& p1, const Pt& p2) {
    int dx = p1.x - p2.x;
    int dy = p1.y - p2.y;
    return sqrt(double(dx * dx + dy * dy));
}

int main() {
    int N, K;
    scanf("%d %d", &N, &K);

    Pt p;
    scanf("%d %d", &p.x, &p.y);

    double ans = 0.0;
    for (int i = 1; i < N; i++) {
        Pt q;
        scanf("%d %d", &q.x, &q.y);
        ans += d(p, q);
        p = q;
    }

    printf("%.10f\n", ans * K / 50.0);

    return 0;
}

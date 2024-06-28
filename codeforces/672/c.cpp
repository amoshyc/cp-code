#include <bits/stdc++.h>
using namespace std;

typedef double dd;
const dd INF = 1e19;
const int MAX_N = 100000;
int N;
dd ax, ay, bx, by, rx, ry;
dd da[MAX_N];
dd db[MAX_N];
dd dr[MAX_N];

inline dd d(dd x1, dd y1, dd x2, dd y2) {
    dd dx = x1 - x2;
    dd dy = y1 - y2;
    return sqrt(dx * dx + dy * dy);
}

dd solve() {
    double ans = 0.0;
    for (int i = 0; i < N; i++)
        ans += dr[i] * 2;

    dd min_a[2] = {INF, INF};
    int id_a[2] = {-1, -1};
    dd min_b[2] = {INF, INF};
    int id_b[2] = {-1, -1};

    // 找最小與次小，分別存於 min_*[0], min_*[1]
    for (int i = 0; i < N; i++) {
        dd disa = -dr[i] + da[i];
        int ida = i;
        if (disa < min_a[0]) {
            swap(disa, min_a[0]);
            swap(ida, id_a[0]);
        }
        if (min_a[0] < disa && disa < min_a[1]) {
            min_a[1] = disa;
            id_a[1] = ida;
        }

        dd disb = -dr[i] + db[i];
        int idb = i;
        if (disb < min_b[0]) {
            swap(disb, min_b[0]);
            swap(idb, id_b[0]);
        }
        if (min_b[0] < disb && disb < min_b[1]) {
            min_b[1] = disb;
            id_b[1] = idb;
        }
    }

    // 看不重複的情況下，怎麼搭配比較小
    double reduce = min(min_a[0], min_b[0]); // one stop at init
    for (int i = 0; i < 2; i++) {
        for (int j = 0; j < 2; j++) {
            if (id_a[i] != id_b[j]) {
                reduce = min(reduce, min_a[i] + min_b[j]);
            }
        }
    }

    return ans + reduce;
}

int main() {
    scanf("%lf %lf %lf %lf %lf %lf", &ax, &ay, &bx, &by, &rx, &ry);
    scanf("%d", &N);

    for (int i = 0; i < N; i++) {
        double x, y;
        scanf("%lf %lf", &x, &y);
        da[i] = d(x, y, ax, ay);
        db[i] = d(x, y, bx, by);
        dr[i] = d(x, y, rx, ry);
    }

    printf("%.10f\n", solve());

    return 0;
}

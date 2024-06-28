#include <bits/stdc++.h>
using namespace std;

// 最大是 1: 1^n 種
// 最大是 2: 2^n - 1^n 種
// 最大是 3: 3^n - 2^n 種
// …
// 最大是 m: m^n - (m - 1)^n 種

typedef double db;

int main() {
    // ios::sync_with_stdio(false);
    // cin.tie(0);

    int M, N;
    scanf("%d %d", &M, &N);

    double expec = 0.0;
    for (int i = 1; i <= M; i++) {
        double p = pow(db(i) / db(M), db(N)) - pow(db(i - 1) / db(M), db(N));
        expec += p * i;
    }

    printf("%.10f\n", expec);

    return 0;
}

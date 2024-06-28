#include <bits/stdc++.h>
using namespace std;

int main() {
    int n, b, p;
    scanf("%d %d %d", &n, &b, &p);

    long long cnt_b = 0, cnt_p = n * p;
    while (n > 1) {
        int nn = 1;
        while (nn < n)
            nn <<= 1;
        if (nn > n)
            nn >>= 1;

        cnt_b += nn * b + nn / 2;

        n = n - nn;
        n += nn / 2;
    }

    printf("%lld %lld\n", cnt_b, cnt_p);

    return 0;
}

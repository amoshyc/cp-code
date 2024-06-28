#include <bits/stdc++.h>
using namespace std;

const int max_n = 100;

int n;
char data[max_n][max_n + 1];

int main() {
    scanf("%d", &n);
    for (int r = 0; r < n; r++) {
        scanf("%s", data[r]);
    }

    long long cnt = 0;

    // row
    for (int r = 0; r < n; r++) {
        long long val = 0;
        for (int c = 0; c < n; c++)
            if (data[r][c] == 'C')
                val++;

        if (val > 1)
            cnt += val * (val - 1) / 2;
    }

    // col
    for (int c = 0; c < n; c++) {
        long long val = 0;
        for (int r = 0; r < n; r++)
            if (data[r][c] == 'C')
                val++;

        if (val > 1)
            cnt += val * (val - 1) / 2;
    }

    printf("%lld\n", cnt);

    return 0;
}

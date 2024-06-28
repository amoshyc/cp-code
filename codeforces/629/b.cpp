#include <bits/stdc++.h>
using namespace std;

int n;
int cnt_m[400];
int cnt_f[400];


int main() {
    scanf("%d\n", &n);
    for (int i = 0; i < n; i++) {
        char g; int a, b;
        scanf("%c %d %d\n", &g, &a, &b);

        if (g == 'M') {
            for (int j = a; j <= b; j++)
                cnt_m[j]++;
        }
        else {
            for (int j = a; j <= b; j++)
                cnt_f[j]++;
        }
    }

    int ans = -1;
    for (int i = 1; i <= 366; i++)
        ans = max(ans, min(cnt_m[i], cnt_f[i]) * 2);

    printf("%d\n", ans);

    return 0;
}

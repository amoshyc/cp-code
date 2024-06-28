#include <bits/stdc++.h>
using namespace std;

const int max_n = 500000;

int n;
int a[2 * max_n + 1];

int main() {
    scanf("%d", &n);

    int idx = 1;
    for (int i = 1; i < n; i += 2) {
        a[idx] = a[idx + n - i] = i;
        idx++;
    }

    idx = n + 1;
    for (int i = 2; i < n; i += 2) {
        a[idx] = a[idx + n - i] = i;
        idx++;
    }

    for (int i = 1; i <= 2 * n; i++)
        if (a[i] == 0)
            a[i] = n;

    for (int i = 1; i <= 2 * n; i++)
        printf("%d ", a[i]);
    puts("");

    return 0;
}

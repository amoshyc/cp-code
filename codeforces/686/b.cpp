#include <bits/stdc++.h>
using namespace std;

int main() {
    int N; scanf("%d", &N);
    vector<int> a(N, 0);
    for (int i = 0; i < N; i++)
        scanf("%d", &a[i]);

    for (int i = 0; i < N; i++) {
        for (int j = 1; j < N; j++) {
            if (a[j] < a[j - 1]) {
                swap(a[j - 1], a[j]);
                printf("%d %d\n", j, j + 1);
            }
        }
    }

    // for (int i = 0; i < N; i++) {
    //     printf("%d ", a[i]);
    // }
    // puts("");

    return 0;
}

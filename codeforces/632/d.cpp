#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, M;
    scanf("%d %d", &N, &M);

    auto A = vector<int>(N, 0);
    // freq[i] = occurrence of i in A
    auto freq = vector<int>(M + 1, 0);
    // cnt[i] = number of items in A that is a factor of i
    auto cnt = vector<int>(M + 1, 0);

    for (int i = 0; i < N; i++) {
        scanf("%d", &A[i]);
    }

    for (int a : A) {
        if (a <= M) {
            freq[a]++;
        }
    }

    for (int i = 1; i <= M; i++) {
        if (freq[i] > 0) {
            for (int j = i; j <= M; j += i) {
                cnt[j] += freq[i];
            }
        }
    }

    int lcm = -1;
    int len = 0;

    for (int i = 1; i <= M; i++) {
        if (cnt[i] > len) {
            len = cnt[i];
            lcm = i;
        }
    }

    if (lcm == -1) { // 無解
        puts("1 0");
        puts("");
    }
    else {
        printf("%d %d\n", lcm, len);
        for (int i = 0; i < N; i++) {
            if (lcm % A[i] == 0) {
                printf("%d ", i + 1);
            }
        }
        puts("");
    }

    return 0;
}

#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int N;
int cnt[1000000 + 50];

int main() {
    scanf("%d", &N);

    int max_w = -1;
    for (int i = 0; i < N; i++) {
        int w; scanf("%d", &w);
        cnt[w]++;
        max_w = max(max_w, w);
    }

    int carry = 0, ans = 0;
    for (int i = 0; i <= max_w || carry != 0; i++) {
        cnt[i] += carry;
        carry = 0;

        carry += cnt[i] / 2;
        cnt[i] %= 2;

        if (cnt[i] == 1)
            ans++;

        // cout << i << " " << cnt[i] << endl;
    }

    printf("%d\n", ans);

    return 0;
}

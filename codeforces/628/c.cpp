#include <bits/stdc++.h>
using namespace std;

const int max_n = 100000;
int n, k;
char input[max_n + 1];
char ans[max_n + 1];

int main() {
    scanf("%d %d", &n, &k);
    scanf("%s", input);

    for (int i = 0; i < n; i++) {
        if (k <= 0) {
            ans[i] = input[i];
            continue;
        }

        int diff1 = input[i] - 'a';
        int diff2 = 'z' - input[i];

        if (diff1 > diff2) {
            if (diff1 <= k) {
                ans[i] = 'a';
                k -= diff1;
            }
            else {
                ans[i] = input[i] - k;
                k = 0;
            }
        }
        else {
            if (diff2 <= k) {
                ans[i] = 'z';
                k -= diff2;
            }
            else {
                ans[i] = input[i] + k;
                k = 0;
            }
        }
    }

    if (k > 0) puts("-1");
    else printf("%s\n", ans);

    return 0;
}

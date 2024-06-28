#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    int N;
    scanf("%d", &N);

    ll move_cnt = 0;
    for (int i = 0; i < N; i++) {
        ll v; scanf("%lld", &v);
        move_cnt += (v - 1);

        if (move_cnt % 2 == 0) {
            puts("2");
        }
        else {
            puts("1");
        }
    }

    return 0;
}

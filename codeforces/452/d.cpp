#include <bits/stdc++.h>
using namespace std;

int K, N[3], T[3];
// priority can be optimised to deque since t[i] is const
deque<int> deq[3];

int main() {
    scanf("%d", &K);
    for (int i = 0; i < 3; i++) scanf("%d", &N[i]);
    for (int i = 0; i < 3; i++) scanf("%d", &T[i]);

    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < N[i]; j++) {
            deq[i].push_back(0);
        }
    }

    for (int i = 0; i < K; i++) {
        int st[3];
        for (int j = 0; j < 3; j++) {
            st[j] = deq[j].front();
            deq[j].pop_front();
        }

        // puts("get:");
        // for (int j : st) {
        //     printf("%d ", j);
        // }
        // puts("");

        // adjusts
        // align to washing machine
        if (st[1] <= st[0] + T[0] && st[2] <= st[0] + T[0] + T[1]) {
            // puts("washing");
            st[1] = st[0] + T[0];
            st[2] = st[1] + T[1];
        }
        // align to drying machine
        else if (st[0] <= st[1] - T[0] && st[2] <= st[1] + T[1]) {
            // puts("drying");
            st[0] = st[1] - T[0];
            st[2] = st[1] + T[1];
        }
        // align to folding machine
        else {
            // puts("folding");
            st[1] = st[2] - T[1];
            st[0] = st[1] - T[0];
        }

        // puts("adjusted:");
        // for (int j : st) {
        //     printf("%d ", j);
        // }
        // puts("");
        // puts("-----------------------");

        for (int i = 0; i < 3; i++) {
            deq[i].push_back(st[i] + T[i]);
        }
    }

    int ans = -1;
    for (int i = 0; i < 3; i++) {
        while (!deq[i].empty()) {
            int ed = deq[i].front();
            deq[i].pop_front();
            ans = max(ans, ed);
        }
    }

    printf("%d\n", ans);

    return 0;
}

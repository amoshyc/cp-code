#include <bits/stdc++.h>
using namespace std;

struct State {
    deque<int> c[2];
    bool operator<(const State &st) const {
        if (c[0] != st.c[0])
            return c[0] < st.c[0];
        return c[1] < st.c[1];
    }
};

map<State, bool> vis;

int main() {
    int N;
    scanf("%d", &N);

    State st;
    for (int i = 0; i < 2; i++) {
        int n;
        scanf("%d", &n);
        for (int j = 0; j < n; j++) {
            int inp;
            scanf("%d", &inp);
            st.c[i].push_back(inp);
        }
    }

    int cnt = 0;
    int win = -1; // 0, 1, 2(loop)
    while (win == -1) {
        // for (int val : st.c[0])
        //     printf("%d ", val);
        // puts("");
        // for (int val : st.c[1])
        //     printf("%d ", val);
        // puts("");
        // puts("-----------------");

        if (st.c[0].size() == 0) {
            win = 1;
            break;
        }
        if (st.c[1].size() == 0) {
            win = 0;
            break;
        }
        if (vis[st]) {
            win = 2;
            break;
        }

        vis[st] = true;

        if (st.c[0].front() > st.c[1].front()) {
            st.c[0].push_back(st.c[1].front());
            st.c[0].push_back(st.c[0].front());
            st.c[1].pop_front();
            st.c[0].pop_front();
        } else {
            st.c[1].push_back(st.c[0].front());
            st.c[1].push_back(st.c[1].front());
            st.c[0].pop_front();
            st.c[1].pop_front();
        }

        cnt++;
    }

    if (win == 2)
        puts("-1");
    else {
        printf("%d %d\n", cnt, win + 1);
    }

    return 0;
}

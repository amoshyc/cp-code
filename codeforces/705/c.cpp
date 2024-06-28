#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    int N, Q;
    scanf("%d %d", &N, &Q);

    int total_unread = 0;
    vector<int> msg_que;
    vector<bool> is_read(Q, false);
    vector<vector<int>> unread_msg(N);
    int idx = -1; // last msg read by (3)

    /*
    2 6
    1 1
    1 2
    1 1
    1 2
    2 2
    3 2
    */

    /*
    2 6
    1 1
    1 1
    1 2
    2 1
    1 1
    3 3
    */

    for (int id = 0; id < Q; id++) {
        int type; scanf("%d", &type);

        if (type == 1) {
            int x; scanf("%d", &x); x--;
            is_read[id] = false;
            total_unread++;
            msg_que.push_back(id);
            unread_msg[x].push_back(id);
        }
        else if (type == 2) {
            int x; scanf("%d", &x); x--;

            for (int msg : unread_msg[x]) {
                if (!is_read[msg]) {
                    is_read[msg] = true;
                    total_unread--;
                }
            }

            unread_msg[x].clear();
        }
        else {
            int t; scanf("%d", &t); t--;
            // [0, idx]
            // [0, t]

            if (t > idx) {
                for (int i = idx + 1; i <= t; i++) {
                    int msg = msg_que[i];
                    if (!is_read[msg]) {
                        total_unread--;
                        is_read[msg] = true;
                    }
                }
                idx = t;
            }
        }

        printf("%d\n", total_unread);
    }

    return 0;
}

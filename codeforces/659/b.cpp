#include <bits/stdc++.h>
using namespace std;

const int max_m = 10000;

typedef pair<int, string> pis;

priority_queue<pis> cnt[max_m];

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N, M;
    cin >> N >> M;
    for (int i = 0; i < N; i++) {
        string name; int r, s;
        cin >> name >> r >> s;
        cnt[r - 1].push(pis(s, name));
    }

    for (int i = 0; i < M; i++) {
        if (cnt[i].size() == 2) {
            pis p1 = cnt[i].top(); cnt[i].pop();
            pis p2 = cnt[i].top(); cnt[i].pop();
            cout << p1.second << " " << p2.second << "\n";
        }
        else {
            pis p1 = cnt[i].top(); cnt[i].pop();
            pis p2 = cnt[i].top(); cnt[i].pop();
            pis p3 = cnt[i].top(); cnt[i].pop();

            if (p2.first == p3.first) {
                cout << "?\n";
            }
            else {
                cout << p1.second << " " << p2.second << "\n";
            }
        }
    }

    return 0;
}

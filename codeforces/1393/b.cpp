#include <iostream>
#include <map>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N; cin >> N;
    map<int, int> cnt;
    for (int i = 0; i < N; i++) {
        int x; cin >> x;
        cnt[x] += 1;
    }

    int has_8 = 0;
    int has_6 = 0;
    int has_4 = 0;
    int has_2 = 0;

    for (auto p : cnt) {
        if (p.second >= 8) {
            has_8 += 1;
            continue;
        }
        if (p.second >= 6) {
            has_6 += 1;
            continue;
        }
        if (p.second >= 4) {
            has_4 += 1;
            continue;
        }
        if (p.second >= 2) {
            has_2 += 1;
            continue;
        }
    }

    int Q; cin >> Q;
    while (Q--) {
        string cmd; int x;
        cin >> cmd >> x;

        if (cmd == "+") {
            if (cnt[x] == 7) {
                has_8 += 1;
                has_6 -= 1;
            }
            if (cnt[x] == 5) {
                has_6 += 1;
                has_4 -= 1;
            }
            if (cnt[x] == 3) {
                has_4 += 1;
                has_2 -= 1;
            }
            if (cnt[x] == 1) {
                has_2 += 1;
            }
            cnt[x] += 1;
        }
        else {
            if (cnt[x] == 2) {
                has_2 -= 1;
            }
            if (cnt[x] == 4) {
                has_4 -= 1;
                has_2 += 1;
            }
            if (cnt[x] == 6) {
                has_6 -= 1;
                has_4 += 1;
            }
            if (cnt[x] == 8) {
                has_8 -= 1;
                has_6 += 1;
            }
            cnt[x] -= 1;
        }

        bool cond0 = has_8 >= 1;
        bool cond1 = has_6 >= 2;
        bool cond2 = has_6 == 1 and (has_4 >= 1 or has_2 >= 1);
        bool cond3 = has_4 >= 2;
        bool cond4 = has_4 == 1 and has_2 >= 2;

        if (cond0 || cond1 || cond2 || cond3 || cond4) {
            cout << "YES" << endl;
        }
        else {
            cout << "NO" << endl;
        }
    }

    return 0;
}
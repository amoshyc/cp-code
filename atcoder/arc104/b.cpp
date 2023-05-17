#include <iostream>
#include <vector>
using namespace std;


int to_int(char c) {
    if (c == 'A') return 0;
    if (c == 'T') return 1;
    if (c == 'C') return 2;
    if (c == 'G') return 3;
    return -1;
}

template <class T> ostream &operator<<(ostream &out, const vector<T> v) {
    for (auto x : v) {
        out << x << " ";
    }
    return out;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    string S;
    cin >> N >> S;

    auto cnt = vector<vector<int>>(N, vector<int>(4, 0));
    cnt[0][to_int(S[0])]++;
    for (int i = 1; i < N; i++) {
        cnt[i] = cnt[i - 1];
        cnt[i][to_int(S[i])]++;
    }


    auto extract_range = [&](int l, int r) {
        auto range_cnt = cnt[r - 1];
        if (l >= 1) {
            for (int i = 0; i < 4; i++) {
                range_cnt[i] -= cnt[l - 1][i];
            }
        }
        return range_cnt;
    };

    int ans = 0;
    for (int i = 0; i < N; i++) {
        for (int l = 2; i + l - 1 < N; l += 2) {
            auto range_cnt = extract_range(i, i + l);
            if (range_cnt[0] == range_cnt[1] and range_cnt[2] == range_cnt[3]) {
                // cout << i << " " << i + l << endl;
                ans++;
            }
        }
    }

    cout << ans << endl;

    return 0;
}
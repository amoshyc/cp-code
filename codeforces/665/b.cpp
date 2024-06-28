#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N, M, K;
    cin >> N >> M >> K;

    deque<int> data;
    for (int i = 0; i < K; i++) {
        int val; cin >> val;
        data.push_back(val);
    }

    ll cnt = 0;
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            int val; cin >> val;
            auto it = find(data.begin(), data.end(), val);

            cnt += (it - data.begin() + 1);
            data.erase(it);
            data.push_front(val);
        }
    }

    cout << cnt << "\n";

    return 0;
}

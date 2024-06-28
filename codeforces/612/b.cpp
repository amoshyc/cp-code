#include <bits/stdc++.h>
using namespace std;

typedef pair<int, int> pii; // <f, s>

int main() {
    ios::sync_with_stdio(false);

    int N; cin >> N;
    vector<pii> data(N);
    for (int i = 0; i < N; i++) {
        data[i].second = i;
        cin >> data[i].first;
    }

    sort(data.begin(), data.end());
    // for (auto p : data)
    //     cout << p.first << ", " << p.second << endl;

    long long cnt = 0;
    for (size_t i = 1; i < data.size(); i++) {
        cnt += abs(data[i].second - data[i-1].second);
    }

    cout << cnt << endl;

    return 0;
}

#include <bits/stdc++.h>
using namespace std;

typedef pair<int, int> pii;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    cin >> N;

    vector<pii> v(N, pii(0, 0));
    for (int i = 0; i < N; i++) {
        int inp; cin >> inp;
        v[i] = pii(inp, i + 1);
    }

    sort(v.begin(), v.end());

    for (int i = 0; i < N / 2; i++) {
        cout << v[i].second << " " << v[N - 1 - i].second << endl;
    }

    return 0;
}

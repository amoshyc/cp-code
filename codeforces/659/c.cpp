#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N, M;
    cin >> N >> M;
    vector<int> a(N, 0);
    for (int i = 0; i < N; i++) {
        cin >> a[i];
    }
    sort(a.begin(), a.end());

    ll cost = 0;
    int idx = 0;
    vector<int> ans;
    for (int i = 1; i <= 1000000000; i++) {
        if (i == a[idx]) {
            idx++;
            continue;
        }

        if (cost + i > M) {
            break;
        }

        cost += i;
        ans.push_back(i);
    }

    cout << ans.size() << "\n";
    for (int i : ans)
        cout << i << " ";
    cout << "\n";

    return 0;
}

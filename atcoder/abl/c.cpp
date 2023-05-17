#include <algorithm>
#include <atcoder/dsu>
#include <iostream>
#include <vector>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N, M;
    cin >> N >> M;

    auto comps = atcoder::dsu(N);
    for (int i = 0; i < M; i++) {
        int u, v;
        cin >> u >> v;
        u--;
        v--;
        comps.merge(u, v);
    }

    int n_comp = comps.groups().size();
    int ans = ((n_comp == 1) ? 0 : n_comp - 1);
    cout << ans << endl;

    return 0;
}
#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

void solve() {
    int N, M;
    cin >> N >> M;

    vector<int> A(N * M);
    for (int i = 0; i < N * M; i++) {
        cin >> A[i];
    }

    int idx = max_element(A.begin(), A.end()) - A.begin();
    int r = idx / M;
    int c = idx % M;

    int area = -1;

    // rect 1
    area = max(area, (r + 1) * (c + 1));
    // rect 2
    area = max(area, (N - r) * (c + 1));
    // rect 3
    area = max(area, (r + 1) * (M - c));
    // rect 4
    area = max(area, (N - r) * (M - c));

    cout << area << "\n";
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int TC;
    cin >> TC;
    while (TC--) {
        solve();
    }

    return 0;
}
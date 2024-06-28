#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, M;
    cin >> N >> M;

    int cnt = 0;
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            int w1, w2; cin >> w1 >> w2;
            if (w1 == 1 || w2 == 1)
                cnt++;
        }
    }

    cout << cnt << "\n";

    return 0;
}

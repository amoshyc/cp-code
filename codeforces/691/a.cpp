#include <bits/stdc++.h>
using namespace std;

int main() {
    int N; cin >> N;
    vector<int> A(N, 0);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }

    if (N == 1) {
        cout << ((A[0] == 1) ? "YES" : "NO") << "\n";
        return 0;
    }
    else {
        int zero_cnt = 0;
        for (int i : A)
            if (i == 0)
                zero_cnt++;

        cout << ((zero_cnt == 1) ? "YES" : "NO") << "\n";
    }

    return 0;
}

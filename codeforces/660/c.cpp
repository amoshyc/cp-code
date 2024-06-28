#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, K;
    cin >> N >> K;

    vector<int> A(N, 0);
    vector<int> S(N + 1, 0);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
        S[i + 1] = S[i] + (A[i] == 0);
    }

    if (S[N] <= K) {
        cout << N << endl;
        for (int i = 0; i < N; i++)
            cout << 1 << " ";
        cout << endl;
        return 0;
    }

    // for (int i : S)
    //     cout << i << ", ";
    // cout << endl;

    int max_len = -1, pos = -1;
    for (int i = 1; i <= N; i++) {
        int j = upper_bound(S.begin() + 1, S.end(), S[i - 1] + K) - S.begin();
        j--;

        // cout << i << ": " << j << endl;

        int len = j - i + 1;
        if (len > max_len) {
            max_len = len;
            pos = i - 1;
        }
    }

    for (int i = 0; i < max_len; i++)
        A[pos + i] = 1;

    cout << max_len << endl;
    for (int i : A)
        cout << i << " ";

    return 0;
}

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N; cin >> N;
    vector<int> data(N, 0);
    for (int i = 0; i < N; i++)
        cin >> data[i];
    sort(data.begin(), data.end());

    int idx = N - 1;
    for (int i = 0; i < N / 2; i++) {
        cout << data[i] << " ";
        cout << data[idx] << " ";
        idx--;
    }

    if (N & 1) cout << data[N / 2] << " ";
    cout << endl;

    return 0;
}

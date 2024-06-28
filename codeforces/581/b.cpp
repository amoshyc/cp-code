#include <bits/stdc++.h>
using namespace std;

int h[100000];
int max_of_right[100000];
int N;

int main() {
    ios::sync_with_stdio(false);

    cin >> N;
    for (int i = 0; i < N; i++)
        cin >> h[i];

    max_of_right[N-1] = -1;
    for (int i = N-2; i >= 0; i--) {
        max_of_right[i] = max(h[i+1], max_of_right[i+1]);
    }

    // for (int i = 0; i < N; i++)
    //     cout << max_of_right[i] << " ";
    // cout << endl;

    for (int i = 0; i < N-1; i++) {
        cout << max(0, max_of_right[i] + 1 - h[i]) << " ";
    }
    cout << "0\n";

    return 0;
}

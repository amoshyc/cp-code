#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, T;
    cin >> N >> T;

    if (T == 10) {
        if (N == 1) cout << "-1\n";
        else {
            cout << "1";
            for (int i = 0; i < N - 1; i++)
                cout << "0";
            cout << endl;
        }
    }
    else {
        for (int i = 0; i < N; i++)
            cout << T;
        cout << endl;
    }

    return 0;
}

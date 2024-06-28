#include <bits/stdc++.h>
using namespace std;

int main() {
    int N;
    cin >> N;

    for (int i = 1; i <= N; i++) {
        if (i % 2 == 1) {
            cout << "I hate ";
        }
        else {
            cout << "I love ";
        }
        cout << ((i == N) ? "it" : "that ");
    }
    cout << endl;

    return 0;
}

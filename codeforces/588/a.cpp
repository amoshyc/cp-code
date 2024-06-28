#include <bits/stdc++.h>
using namespace std;

int N;
int p[100000];
int a[100000];

int main() {
    cin >> N;
    for (int i = 0; i < N; i++) {
        cin >> a[i] >> p[i];
    }

    int m = p[0];
    long long total = p[0] * a[0];
    for (int i = 1; i < N; i++) {
        m = min(m, p[i]);
        total += m * a[i];
    }
    cout << total << endl;

    return 0;
}

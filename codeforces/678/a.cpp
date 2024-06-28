#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, K; cin >> N >> K;
    int R = N % K;
    cout << N + (K - R) << endl;
    return 0;
}

#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    cin >> N;
    vector<int> A(N, -1);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }

    prev_permutation(A.begin(), A.end());

    for (int a : A) {
        cout << a << " ";
    }
    cout << endl;

    return 0;
}
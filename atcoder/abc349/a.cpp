#include <algorithm>
#include <iostream>
#include <numeric>
#include <vector>

using namespace std;
using i64 = long long;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    cin >> n;
    vector<i64> arr(n, 0);
    for (auto &x : arr) {
        cin >> x;
    }

    i64 sum = accumulate(arr.begin(), arr.end(), 0);
    cout << -sum << endl;

    return 0;
}
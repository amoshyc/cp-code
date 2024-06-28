#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N; cin >> N;
    vector<int> data(N, 0);
    for (int i = 0; i < N; i++)
        cin >> data[i];

    int MM = max_element(data.begin(), data.end()) - data.begin();
    int mm = min_element(data.begin(), data.end()) - data.begin();

    int res[4];
    res[0] = MM - 0;
    res[1] = N - 1 - MM;
    res[2] = mm - 0;
    res[3] = N - 1 - mm;

    cout << *max_element(res, res + 4) << endl;

    return 0;
}

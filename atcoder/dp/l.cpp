#include <iostream>
#include <tuple>
#include <unordered_map>
#include <vector>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    cin >> N;
    vector<int> A(N);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }

    using ll = long long;
    using pii = pair<ll, ll>;
    auto dp =
        vector<vector<vector<pii>>>(N, vector<vector<pii>>(N, vector<pii>(2)));

    for (int i = 0; i < N; i++) {
        dp[i][i][1] = {A[i], 0};
        dp[i][i][0] = {0, A[i]};
    }

    for (int l = 2; l <= N; l++) {
        for (int i = 0; i + l <= N; i++) {
            int j = i + l - 1;

            {
                auto [X1, Y1] = dp[i + 1][j][0];
                X1 += A[i];
                auto [X2, Y2] = dp[i][j - 1][0];
                X2 += A[j];
                dp[i][j][1] = ((X1 - Y1 > X2 - Y2) ? pii(X1, Y1) : pii(X2, Y2));
            }

            {
                auto [X1, Y1] = dp[i + 1][j][1];
                Y1 += A[i];
                auto [X2, Y2] = dp[i][j - 1][1];
                Y2 += A[j];
                dp[i][j][0] = ((X1 - Y1 < X2 - Y2) ? pii(X1, Y1) : pii(X2, Y2));
            }
        }
    }

    auto [X, Y] = dp[0][N - 1][1];
    cout << X - Y << endl;

    return 0;
}
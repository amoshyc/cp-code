#include <iostream>
#include <algorithm>
#include <vector>

using namespace std;

vector<vector<int>> transpose(const vector<vector<int>>& A) {
    int N = A.size();
    int M = A[0].size();
    auto res = vector<vector<int>>(M, vector<int>(N, 0));
    for (int r = 0; r < N; r++) {
        for (int c = 0; c < M; c++) {
            res[c][r] = A[r][c];
        }
    }
    return res;
}

int popcount(int x) {
    return __builtin_popcount(x);
}

bool is_valid(int prev_mask, int curr_mask, int M) {
    int bits = prev_mask ^ curr_mask;
    if (M == 2) {
        return (bits != 0b00 and bits != 0b11);
    }
    else {
        bool check1 = (bits != 0b000 and bits != 0b111);
        bool check2 = (bits != 0b110 and bits != 0b011);
        bool check3 = (bits != 0b001 and bits != 0b100);
        return (check1 and check2 and check3);
    }
}

int solve() {
    int N, M;
    cin >> N >> M;

    auto A = vector<vector<int>>(N, vector<int>(M, 0));
    for (int r = 0; r < N; r++) {
        string s; cin >> s;
        for (int c = 0; c < M; c++) {
            A[r][c] = int(s[c] - '0');
        }
    }

    if (N >= 4 and M >= 4) {
        return -1;
    }

    if (N == 1 or M == 1) {
        return 0;
    }

    if (N < M) {
        A = transpose(A);
        swap(N, M);
    }

    // Nx2 or Nx3

    auto state = vector<int>(N, 0);
    for (int r = 0; r < N; r++) {
        for (int c = 0; c < M; c++) {
            if (A[r][c] == 1) {
                state[r] |= (1 << (M - 1 - c));
            }
        }
    }

    auto dp = vector<vector<int>>(N, vector<int>(1 << M, N * M));
    for (int mask = 0; mask < (1 << M); mask++) {
        dp[0][mask] = popcount(state[0] ^ mask);
    }

    for (int r = 1; r < N; r++) {
        for (int curr_mask = 0; curr_mask < (1 << M); curr_mask++) {
            for (int prev_mask = 0; prev_mask < (1 << M); prev_mask++) {
                if (is_valid(prev_mask, curr_mask, M)) {
                    dp[r][curr_mask] = min(dp[r][curr_mask], dp[r - 1][prev_mask]);
                }
            }
            dp[r][curr_mask] += popcount(state[r] ^ curr_mask);
        }
    }

    return *min_element(dp[N - 1].begin(), dp[N - 1].end());
}


int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    cout << solve() << endl;
    return 0;
}
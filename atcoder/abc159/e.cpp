#include <bits/stdc++.h>
using namespace std;

void print_vector(const vector<int>& v) {
    for (int x : v) {
        cout << x << " ";
    }
    cout << endl;
}

int H, W, K;
string A[10];

int check(const vector<int>& pivots) {
    // cout << "pivots: ";
    // print_vector(pivots);

    int cut_cnt = 0;
    int n_block = pivots.size() - 1;
    auto val = vector<int>(n_block, 0);

    for (int c = 0; c < W; c++) {
        // cout << "col " << c << ":" << endl;
        vector<int> add(n_block, 0);
        for (int block_id = 0; block_id < n_block; block_id++) {
            int block_start = pivots[block_id];
            int block_end = pivots[block_id + 1];
            // cout << "block " << block_start << "," << block_end << endl;
            for (int r = block_start; r < block_end; r++) {
                add[block_id] += A[r][c] - '0';
            }
        }

        // cout << "val:";
        // print_vector(val);
        // cout << "add:";
        // print_vector(add);

        for (int block_id = 0; block_id < n_block; block_id++) {
            if (add[block_id] > K) {
                // cout << "impossible" << endl;
                return H * W + 100;
            }
        }

        bool cut = false;
        for (int block_id = 0; block_id < n_block; block_id++) {
            if (val[block_id] + add[block_id] > K) {
                cut = true;
                cut_cnt++;
                break;
            }
        }

        for (int block_id = 0; block_id < n_block; block_id++) {
            if (cut) {
                val[block_id] = add[block_id];
            }
            else {
                val[block_id] = val[block_id] + add[block_id];
            }
        }
    }
    // cout << "cut_cnt:" << cut_cnt << endl;

    return cut_cnt + n_block - 1;
}

int solve() {
    int ans = H * W + 100;
    for (int mask = 0; mask < (1 << (H + 1)); mask++) {
        if ((mask & (1 << 0)) == 0) continue;
        if ((mask & (1 << H)) == 0) continue;
        auto pivots = vector<int>();
        for (int r = 0; r <= H; r++) {
            if (mask & (1 << r)) {
                pivots.push_back(r);
            }
        }

        ans = min(ans, check(pivots));
    }

    return ans;
}

int main() {
    ios::sync_with_stdio(false);

    cin >> H >> W >> K;
    for (int i = 0; i < H; i++) {
        cin >> A[i];
    }

    cout << solve() << endl;

    return 0;
}
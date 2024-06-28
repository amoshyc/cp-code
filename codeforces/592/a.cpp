#include <bits/stdc++.h>
using namespace std;

int main() {
    string inp[8];
    for (int i = 0; i < 8; i++)
        cin >> inp[i];

    int min_A_step = 10, min_B_step = 10;

    for (int row = 0; row < 8; row++) {
        for (int col = 0; col < 8; col++) {
            if (inp[row][col] == 'B') { // B
                bool block_by_A = false;
                for (int r = row; r < 8; r++) {
                    if (inp[r][col] == 'W') {
                        block_by_A = true;
                        break;
                    }
                }

                if (!block_by_A)
                    min_B_step = min(min_B_step, 8 - row - 1);
            }
            else if (inp[row][col] == 'W') { // A
                bool block_by_B = false;
                for (int r = row; r >= 0; r--) {
                    if (inp[r][col] == 'B') {
                        block_by_B = true;
                        break;
                    }
                }

                if (!block_by_B)
                    min_A_step = min(min_A_step, row);
            }
        }
    }

    // cout << min_A_step << " " << min_B_step << endl;

    if (min_A_step <= min_B_step)
        cout << "A\n";
    else
        cout << "B\n";

    return 0;
}

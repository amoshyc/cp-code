#include <bits/stdc++.h>
using namespace std;

struct Rec {
    int l1, l2;
};

Rec rect[3];
Rec org[3];

int ans[100][100];
int N;

bool C(int id1, int id2, int id3) {
    // int w1 = min(rect[id1].l1, rect[id1].l2);
    // int h1 = max(rect[id1].l1, rect[id1].l2);
    // int w2 = min(rect[id2].l1, rect[id2].l2);
    // int h2 = max(rect[id2].l1, rect[id2].l2);
    // int w3 = min(rect[id3].l1, rect[id3].l2);
    // int h3 = max(rect[id3].l1, rect[id3].l2);
    int w1 = rect[id1].l1, h1 = rect[id1].l2;
    int w2 = rect[id2].l1, h2 = rect[id2].l2;
    int w3 = rect[id3].l1, h3 = rect[id3].l2;

    // printf("%d, %d, %d\n", h1, h2, h3);
    // type1
    if (w1 + w2 + w3 == h1 && h1 == h2 && h2 == h3) {
        N = h3;
        for (int row = 0; row < N; row++) {
            for (int col = 0; col < N; col++) {
                if (col < w1)
                    ans[row][col] = id1;
                else if (col < w1 + w2)
                    ans[row][col] = id2;
                else
                    ans[row][col] = id3;
            }
        }
        // printf("%d, %d, %d\n", w1, w2, w3);
        return true;
    }

    // type 2
    // printf("(%d, %d), (%d, %d), (%d, %d)\n", w1, h1, w2, h2, w3, h3);
    if (w1 + w2 == w3 && h1 == h2 && h1 + h3 == w3) {
        N = w3;
        for (int col = 0; col < N; col++) {
            for (int row = 0; row < h3; row++)
                ans[row][col] = id3;
            for (int row = h3; row < N; row++) {
                if (col < w1)
                    ans[row][col] = id1;
                else
                    ans[row][col] = id2;
            }
        }
        return true;
    }

    return false;
}

void solve() {
    bool is_possible = false;
    // rotate
    for (int r = 0; r < 8; r++) {
        for (int i = 0; i < 3; i++)
            rect[i] = org[i];

        if (r & 1)
            swap(rect[0].l1, rect[0].l2);
        if ((r >> 1) & 1)
            swap(rect[1].l1, rect[1].l2);
        if ((r >> 2) & 1)
            swap(rect[2].l1, rect[2].l2);

        // order
        if (C(0, 1, 2) || C(0, 2, 1) || C(1, 0, 2) || C(1, 2, 0) ||
            C(2, 0, 1) || C(2, 1, 0)) {
            is_possible = true;
            break;
        }
    }

    string decode = "ABC";
    if (is_possible) {
        printf("%d\n", N);
        for (int row = 0; row < N; row++) {
            for (int col = 0; col < N; col++)
                printf("%c", decode[ans[row][col]]);
            printf("\n");
        }
    }
    else
        puts("-1");
}

int main() {
    for (int i = 0; i < 3; i++)
        scanf("%d %d", &org[i].l1, &org[i].l2);
    solve();

    return 0;
}

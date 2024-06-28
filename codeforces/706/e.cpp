#include <bits/stdc++.h>
using namespace std;

const int MAX_N = 1000;
const int MAX_M = 1000;
int N, M, Q;
int val[(MAX_N + 1) * (MAX_M + 1)]; // val[id] = value of item id
int nxtR[(MAX_N + 1) * (MAX_M + 1)]; // nxtR[id]
int nxtC[(MAX_N + 1) * (MAX_M + 1)]; // nxtC[id]

inline int get_id(int r, int c) {
    if (r < 0 || c < 0 || r > N || c > M)
        return -1;
    return r * (M + 1) + c;
}

inline int get_item_at(int r, int c) {
    int id = get_id(0, 0);
    while (r--) id = nxtR[id];
    while (c--) id = nxtC[id];
    return id;
}

vector<int> get_vborder(int r, int c, int h) {
    vector<int> res(h);
    int id = get_item_at(r, c);
    for (int i = 0; i < h; i++) {
        res[i] = id;
        id = nxtR[id];
    }
    return res;
}

vector<int> get_hborder(int r, int c, int w) {
    vector<int> res(w);
    int id = get_item_at(r, c);
    for (int i = 0; i < w; i++) {
        res[i] = id;
        id = nxtC[id];
    }
    return res;
}

void matrix_swap(int r1, int c1, int r2, int c2, int h, int w) {
    vector<int> borders1[4];
    borders1[0] = get_vborder(r1, c1 - 1, h); // left
    borders1[1] = get_vborder(r1, c1 + w - 1, h); // right
    borders1[2] = get_hborder(r1 - 1, c1, w); // top
    borders1[3] = get_hborder(r1 + h - 1, c1, w); // bottom
    vector<int> borders2[4];
    borders2[0] = get_vborder(r2, c2 - 1, h); // left
    borders2[1] = get_vborder(r2, c2 + w - 1, h); // right
    borders2[2] = get_hborder(r2 - 1, c2, w); // top
    borders2[3] = get_hborder(r2 + h - 1, c2, w); // bottom

    for (int i = 0; i < h; i++) {
        swap(nxtC[borders1[0][i]], nxtC[borders2[0][i]]); // left
        swap(nxtC[borders1[1][i]], nxtC[borders2[1][i]]); // right
    }
    for (int i = 0; i < w; i++) {
        swap(nxtR[borders1[2][i]], nxtR[borders2[2][i]]); // top
        swap(nxtR[borders1[3][i]], nxtR[borders2[3][i]]); // bottom
    }
}

int main() {
    scanf("%d %d %d", &N, &M, &Q);
    for (int r = 1; r <= N; r++) {
        for (int c = 1; c <= M; c++) {
            scanf("%d", &val[get_id(r, c)]);
        }
    }

    for (int r = 0; r <= N; r++) {
        for (int c = 0; c <= M; c++) {
            nxtR[get_id(r, c)] = get_id(r + 1, c);
            nxtC[get_id(r, c)] = get_id(r, c + 1);
        }
    }

    while (Q--) {
        int r1, c1, r2, c2, h, w;
        scanf("%d %d %d %d %d %d", &r1, &c1, &r2, &c2, &h, &w);
        matrix_swap(r1, c1, r2, c2, h, w);
    }

    for (int r = get_item_at(1, 1); r != -1; r = nxtR[r]) {
        for (int c = r; c != -1; c = nxtC[c]) {
            printf("%d ", val[c]);
        }
        puts("");
    }

    return 0;
}

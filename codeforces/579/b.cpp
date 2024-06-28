#include <bits/stdc++.h>
using namespace std;

struct Node {
    int row, col, w;
    bool operator > (const Node& n) const {
        return w > n.w;
    }
};

int main() {
    int N;
    scanf("%d", &N);
    N = N * 2;

    vector<Node> v;
    for (int row = 2; row <= N; row++) {
        for (int col = 1; col < row; col++) {
            int w; scanf("%d", &w);
            v.push_back(Node{row, col, w});
        }
    }

    sort(v.begin(), v.end(), greater<Node>());

    vector<int> match(N+1, -1);
    for (const Node& node : v) {
        // printf("%d %d %d\n", node.row, node.col, node.w);
        if (match[node.row] == -1 && match[node.col] == -1) {
            match[node.row] = node.col;
            match[node.col] = node.row;
        }
    }

    for (int i = 1; i <= N; i++) {
        if (i != 1)
            printf(" ");
        printf("%d", match[i]);
    }
    printf("\n");

    return 0;
}

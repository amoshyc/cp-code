#include <bits/stdc++.h>
using namespace std;

typedef unsigned int ui;

struct Trie {
    struct Node {
        int cnt;
        int nxt[2];
        Node() {
            cnt = 0;
            nxt[0] = nxt[1] = -1;
        }
    };

    int NN;
    vector<Node> nodes;
    
    void init(int N) {
        NN = 1;
        nodes.clear();
        nodes.resize(N * 30 + 1, Node()); // notice root
    }

    void insert(int x, int u = 0) {
        for (int i = 30; i >= 0; i--) {
            int b = ((x >> i) & 1);
            if (nodes[u].nxt[b] == -1) {
                nodes[u].nxt[b] = NN++;
            }
            u = nodes[u].nxt[b];
            nodes[u].cnt++;
        }
    }

    void remove(int x, int u = 0) {
        for (int i = 30; i >= 0; i--) {
            int b = ((x >> i) & 1);
            u = nodes[u].nxt[b];
            nodes[u].cnt--;
        }
    }

    int minimize(int x, int u = 0) {
        int res = 0;
        for (int i = 30; i >= 0; i--) {
            int b = ((x >> i) & 1);
            int v = nodes[u].nxt[b];
            if (v != -1 && nodes[v].cnt > 0) {
                u = v;
            } else {
                u = nodes[u].nxt[b ^ 1];
                res |= (1 << i);
            }
        }
        return res;
    }

    int maximize(int x, int u = 0) {
        int res = 0;
        for (int i = 30; i >= 0; i--) {
            int b = ((x >> i) & 1);
            int v = nodes[u].nxt[b ^ 1];
            if (v != -1 && nodes[v].cnt > 0) {
                u = v;
                res |= (1 << i);
            } else {
                u = nodes[u].nxt[b];
            }
        }
        return res;
    }

    void pr(int u = 0, string ind = "", int val = 0) {
        if (u == -1) return;
        printf("%s%d,%d: %d(%d)\n", ind.c_str(), val & 1, u, val, nodes[u].cnt);
        pr(nodes[u].nxt[0], ind + " ", (val << 1) + 0);
        pr(nodes[u].nxt[1], ind + " ", (val << 1) + 1);
    }
};

int main() {
    int Q;
    scanf("%d", &Q);

    Trie trie;
    trie.init(Q);
    trie.insert(0);

    while (Q--) {
        char cmd[10]; int x;
        scanf("%s %d", cmd, &x);

        if (cmd[0] == '+') {
            trie.insert(x);
            continue;
        }
        if (cmd[0] == '-') {
            trie.remove(x);
            continue;
        }

        printf("%d\n", trie.maximize(x));
    }

    return 0;
}
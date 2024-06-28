#include <bits/stdc++.h>
using namespace std;

#define st first
#define nd second
typedef pair<int, int> pii;

const int MAX_N = 200000 + 10;
const int INF = 0x3f3f3f3f;
int N;
int d[MAX_N]; // distance
int s[MAX_N]; // shortcut

bool added[MAX_N];
void bfs() {
    queue<pii> q;
    for (int i = 0; i < N; i++)
        d[i] = INF;

    q.push(pii(0, 0));
    added[0] = true;

    while (!q.empty()) {
        pii front = q.front(); q.pop();
        int l = front.st;
        int v = front.nd;

        d[v] = l;

        // prev number
        if (v - 1 >= 0 && !added[v - 1]) {
            q.push(pii(l + 1, v - 1));
            added[v - 1] = true;
        }

        // next number
        if (v + 1 < N && !added[v + 1]) {
            q.push(pii(l + 1, v + 1));
            added[v + 1] = true;
        }
        // shortcut
        if (s[v] != v && !added[s[v]]) {
            q.push(pii(l + 1, s[v]));
            added[s[v]] = true;
        }
    }
}

int main() {
    scanf("%d", &N);
    for (int i = 0; i < N; i++) {
        scanf("%d", &s[i]); s[i]--;
    }

    bfs();
    for (int i = 0; i < N; i++) {
        printf("%d ", d[i]);
    }
    puts("");

    return 0;
}

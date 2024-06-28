#include <bits/stdc++.h>
using namespace std;

int N, K;
int A[100000];

typedef pair<int, int> pii; // <diff, idx>

int solve() {
    priority_queue< pii, vector<pii>, greater<pii> > pq;
    for (int i = 0; i < N; i++)
        if (A[i] < 100)
            pq.push(pii(10 - (A[i] % 10), i));

    while (!pq.empty() && K > 0) {
        pii node = pq.top(); pq.pop();
        int diff = node.first;
        int idx = node.second;

        if (K > diff) {
            A[idx] += diff;
            K -= diff;
        }
        else {
            A[idx] += K;
            K = 0;
            break;
        }

        if (A[idx] < 100)
            pq.push(pii(10, idx));
    }

    // for (int i = 0; i < N; i++)
    //     cout << A[i] << " ";
    // cout << endl;

    int cnt = 0;
    for (int i = 0; i < N; i++)
        cnt += (A[i] / 10);
    return cnt;
}

int main() {
    scanf("%d %d", &N, &K);
    for (int i = 0; i < N; i++)
        scanf("%d", &A[i]);
    printf("%d\n", solve());

    return 0;
}

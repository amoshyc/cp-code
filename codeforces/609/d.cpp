#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

struct Item {
    int id;
    ll cost;
    bool operator < (const Item& other) const {
        return cost < other.cost;
    }
};

const ll INF = (1LL << 40);
const int max_n = 2 * 100000;

int N, M, K, S;
int A[max_n];
int B[max_n];
vector<Item> items1, items2;

int pmia[max_n]; // prefix min idx of A
int pmib[max_n]; // prefix min idx of B
int day1, day2;
vector<int> buy1, buy2;

bool C(int mid) {
    day1 = pmia[mid];
    day2 = pmib[mid];

    buy1.clear();
    buy2.clear();

    size_t idx1 = 0, idx2 = 0;
    ll money = 0;
    for (int i = 0; i < K; i++) {
        ll p1 = items1[idx1].cost * A[day1];
        ll p2 = items2[idx2].cost * B[day2];

        if (p1 < p2) {
            money += p1;
            buy1.push_back(items1[idx1++].id);
        }
        else {
            money += p2;
            buy2.push_back(items2[idx2++].id);
        }
    }

    return money <= S;
}

int main() {
    scanf("%d %d %d %d", &N, &M, &K, &S);
    for (int i = 0; i < N; i++)
        scanf("%d", &A[i]);
    for (int i = 0; i < N; i++)
        scanf("%d", &B[i]);
    for (int i = 0; i < M; i++) {
        int type; ll cost;
        scanf("%d %lld", &type, &cost);
        if (type == 1) items1.push_back(Item{i+1, cost});
        else items2.push_back(Item{i+1, cost});
    }

    // build pmia, pmib
    pmia[0] = 0;
    pmib[0] = 0;
    for (int i = 1; i < N; i++) {
        pmia[i] = ((A[i] < A[pmia[i-1]]) ? i : pmia[i-1]);
        pmib[i] = ((B[i] < B[pmib[i-1]]) ? i : pmib[i-1]);
    }

    // sort and add stop indicator to each items.
    sort(items1.begin(), items1.end());
    sort(items2.begin(), items2.end());
    items1.push_back(Item{-1, INF});
    items2.push_back(Item{-1, INF});

    // 0 0 0 0 1 1 1
    // (lb, ub]
    int lb = -1, ub = N;
    while (ub - lb > 1) {
        int mid = (lb + ub) / 2;
        if (C(mid)) ub = mid;
        else lb = mid;
    }

    if (ub == N) printf("-1\n");
    else {
        // +1 to make day start from 1 instead of 0
        C(ub);
        printf("%d\n", ub + 1);
        for (int id : buy1)
            printf("%d %d\n", id, day1 + 1);
        for (int id : buy2)
            printf("%d %d\n", id, day2 + 1);
    }

    return 0;
}

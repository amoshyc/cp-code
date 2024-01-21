#include <iostream>
#include <iterator>
#include <set>
#include <vector>
#include <algorithm>
using namespace std;
    
int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    
    int N, Q;
    int M = 2 * 100000;
    cin >> N >> Q;
    
    auto belong = vector<int>(N, -1);
    auto rating = vector<int>(N, -1);
    auto evennesses = multiset<int>();
    auto kdrgrtns = vector<multiset<int>>(M);
    
    for (int i = 0; i < N; i++) {
        int A, B; 
        cin >> A >> B;
        B--;
        rating[i] = A;
        belong[i] = B;
        kdrgrtns[B].insert(A);
    }
    for (auto kdrgrtn: kdrgrtns) {
        if (!kdrgrtn.empty()) {
            evennesses.insert(*kdrgrtn.rbegin());
        }
    }
    
    while (Q--) {
        int C, D;
        cin >> C >> D;
        C--; D--;
    
        auto &kdrgrtn_src = kdrgrtns[belong[C]];
        auto &kdrgrtn_dst = kdrgrtns[D];
    
        // Note that do not use evennesses.erase(val);
        // Use evennesses.erase(evennesses.find(val)) instead.
        // The former will erase all entries that is val while the latter erase a single entry
        evennesses.erase(evennesses.find(*kdrgrtn_src.rbegin()));
        kdrgrtn_src.erase(kdrgrtn_src.find(rating[C]));
        if (kdrgrtn_src.size() > 0) {
            evennesses.insert(*kdrgrtn_src.rbegin());
        }

        if (kdrgrtn_dst.size() > 0) {
            evennesses.erase(evennesses.find(*kdrgrtn_dst.rbegin()));
        }
        kdrgrtn_dst.insert(rating[C]);
        evennesses.insert(*kdrgrtn_dst.rbegin());
    
        belong[C] = D;
    
        cout << *evennesses.begin() << endl;
    }
    
    return 0;
}
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
     
        // auto check = [&](){
        //     cout << "kdrgtns:" << endl;
        //     for (int i = 0; i < M; i++) {
        //         auto kdrgrtn = kdrgrtns[i];
        //         if (kdrgrtn.size() > 0) {
        //             cout << "  " << i << ": ";
        //             copy(kdrgrtn.begin(), kdrgrtn.end(), ostream_iterator<int>(cout, ", "));
        //             cout << endl;
        //         }
        //     }
     
        //     cout << "belong: ";
        //     copy(belong.begin(), belong.end(), ostream_iterator<int>(cout, ", "));
        //     cout << endl;
     
        //     cout << "evenesses: ";
        //     copy(evennesses.begin(), evennesses.end(), ostream_iterator<int>(cout, ", "));
        //     cout << endl;
        // };
     
        // check();
     
        while (Q--) {
            int C, D;
            cin >> C >> D;
            C--; D--;
     
            auto &kdrgrtn_src = kdrgrtns[belong[C]];
            auto &kdrgrtn_dst = kdrgrtns[D];
     
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
            // cout << "-------" << endl;
            // cout << C << "," << D << endl;
            // check();
     
            cout << *evennesses.begin() << endl;
        }
     
        return 0;
    }
#include <algorithm>
#include <iostream>
#include <list>
#include <vector>
#define sz(x) (int(x.size()))
using namespace std;

template <class T> ostream &operator<<(ostream &out, const vector<T> v) {
    out << "[ ";
    for (auto x : v) {
        out << x << " ";
    }
    out << "]";
    return out;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N, M, K, R;
    cin >> N >> M >> K >> R;
    vector<vector<int>> D(N, vector<int>(K, 0));
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < K; j++) {
            cin >> D[i][j];
        }
    }

    vector<vector<int>> G(N);
    vector<int> indeg(N, 0);
    for (int i = 0; i < R; i++) {
        int u, v;
        cin >> u >> v;
        u--;
        v--;
        G[u].push_back(v);
        indeg[v]++;
    }

    vector<list<int>> pending(M, list<int>());
    vector<int> assign(N, 0);
    vector<int> start(N, -1);
    vector<int> duration(N, -1);
    vector<string> status(N, "init");

    int n_complete = 0;
    int day = 0;

    const int N_TRAIN = N;

    auto dist = [&](int a, int b) -> int {
        int res = 0;
        for (int k = 0; k < K; k++) {
            if (D[b][k] < D[a][k]) {
                res += D[a][k] - D[b][k];
            }
        }
        return res;
    };

    auto apply = [&](int day) -> void {
        vector<pair<int, int>> today;
        for (int user_id = 0; user_id < M; user_id++) {
            if (sz(pending[user_id]) == 0)
                continue;
            int task_id = pending[user_id].front();
            if (status[task_id] == "ready") {
                status[task_id] = "working";
                start[task_id] = day;
                today.push_back({user_id, task_id});
            }
        }
        cout << sz(today);
        for (auto [uid, tid] : today) {
            cout << " " << uid + 1 << " " << tid + 1;
        }
        cout << endl;
        cout.flush();
    };

    auto update = [&](int day) -> void {
        int n;
        cin >> n;
        if (n == -1) {
            exit(0);
        }
        while (n--) {
            int f;
            cin >> f;
            int user_id = f - 1;
            int task_id = pending[user_id].front();
            pending[user_id].pop_front();

            status[task_id] = "done";
            duration[task_id] = day - start[task_id] + 1;
            n_complete++;
            for (auto v : G[task_id]) {
                indeg[v]--;
            }
        }
    };

    // stage 1
    for (; day < 2000; day++) {
        // plan
        vector<int> task_ids;
        for (int task_id = 0; task_id < N; task_id++) {
            if (indeg[task_id] == 0 and status[task_id] == "init") {
                task_ids.push_back(task_id);
                if (n_complete + sz(task_ids) >= N_TRAIN) {
                    break;
                }
            }
        }
        for (auto task_id : task_ids) {
            int user_id = -1;
            for (int uid = 0; uid < M; uid++) {
                if (sz(pending[uid]) == 0) {
                    user_id = uid;
                    break;
                }
            }
            if (user_id != -1) {
                pending[user_id].push_back(task_id);
                status[task_id] = "ready";
                assign[task_id] = user_id;
            }
        }

        apply(day);
        update(day);
        if (n_complete >= N_TRAIN) {
            break;
        }
    }

    vector<vector<int>> prediction(N, vector<int>(M, 1e9));
    vector<int> train_task_ids;
    for (int task_id = 0; task_id < N; task_id++) {
        if (status[task_id] == "done") {
            train_task_ids.push_back(task_id);
        }
    }

    for (int task_id = 0; task_id < N; task_id++) {
        if (status[task_id] == "done")
            continue;
        for (auto train_id : train_task_ids) {
            int user_id = assign[train_id];
            int dist2train = dist(task_id, train_id) + duration[train_id];
            if (dist2train < prediction[task_id][user_id]) {
                prediction[task_id][user_id] = dist2train;
            }
        }
    }

    // stage 2
    for (; day < 2000; day++) {
        vector<int> task_ids;
        for (int task_id = 0; task_id < N; task_id++) {
            if (indeg[task_id] == 0 and status[task_id] == "init") {
                task_ids.push_back(task_id);
            }
        }

        vector<int> pending_time(M, 0);
        for (int user_id = 0; user_id < M; user_id++) {
            for (auto task_id : pending[user_id]) {
                pending_time[user_id] += prediction[task_id][user_id];
            }
        }

        for (auto task_id : task_ids) {
            int user_id = -1;
            int min_time = 1e9;
            for (int uid = 0; uid < M; uid++) {
                int time = pending_time[uid] + prediction[task_id][uid];
                if (time < min_time) {
                    min_time = time;
                    user_id = uid;
                }
            }

            pending[user_id].push_back(task_id);
            pending_time[user_id] += prediction[task_id][user_id];
            status[task_id] = "ready";
            assign[task_id] = user_id;
        }

        apply(day);
        update(day);
    }

    return 0;
}
#include <algorithm>
#include <cmath>
#include <cstdlib>
#include <iostream>
#include <queue>
#include <random>
#include <utility>
#include <vector>
#define sz(x) (int(x.size()))
using namespace std;

default_random_engine generator(42);

struct R {
    int x, y, r;
};
struct B {
    int x1, y1, x2, y2;

    int area() const { return (x2 - x1) * (y2 - y1); }

    void clip_() {
        x1 = max(0, min(x1, 10000));
        y1 = max(0, min(y1, 10000));
        x2 = max(0, min(x2, 10000));
        y2 = max(0, min(y2, 10000));
    }

    B operator+(const B &b) {
        return B{x1 + b.x1, y1 + b.y1, x2 + b.x2, y2 + b.y2};
    }
    B operator-(const B &b) {
        return B{x1 - b.x1, y1 - b.y1, x2 - b.x2, y2 - b.y2};
    }
};

bool has_overlap(const vector<B> &boxes, int idx) {
    const auto boxes_a = boxes[idx];
    for (int i = 0; i < sz(boxes); i++) {
        if (i == idx)
            continue;
        const auto boxes_b = boxes[i];
        auto dx = min(boxes_a.x2, boxes_b.x2) - max(boxes_a.x1, boxes_b.x1);
        auto dy = min(boxes_a.y2, boxes_b.y2) - max(boxes_a.y1, boxes_b.y1);
        if (dx > 0 && dy > 0) {
            return true;
        }
    }
    return false;
}

vector<B> solve(const vector<R> &requests) {
    int N = sz(requests);
    vector<B> boxes;
    for (int i = 0; i < N; i++) {
        auto [x, y, r] = requests[i];
        boxes.push_back({x, y, x + 1, y + 1});
    }

    for (int t = 0; t < 10000; t++) {
        vector<double> prob(N, 0.0);
        for (int i = 0; i < N; i++) {
            auto [x1, y1, x2, y2] = boxes[i];
            double area = double(x2 - x1 + 1) * double(y2 - y1 + 1);
            double r = requests[i].r;
            prob[i] = max(double(0.0), log(r) - log(area));
        }
        discrete_distribution<int> distribution(prob.begin(), prob.end());
        int idx = distribution(generator);
        // int idx = random() % N;

        int l = 1;
        if (t <= 2500) {
            l = 100;
        } else if (t <= 5000) {
            l = 50;
        } else if (t <= 7500) {
            l = 10;
        }

        B increments[4] = {
            {-l, 0, 0, 0}, {0, -l, 0, 0}, {0, 0, +l, 0}, {0, 0, 0, +l}};
        shuffle(increments, increments + 4, generator);

        for (auto inc : increments) {
            boxes[idx] = boxes[idx] + inc;
            if (has_overlap(boxes, idx) == false) {
                boxes[idx].clip_();
                break;
            }
            boxes[idx] = boxes[idx] - inc;
        }
    }

    return boxes;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    cin >> N;
    vector<R> requests;
    for (int i = 0; i < N; i++) {
        int x, y, r;
        cin >> x >> y >> r;
        requests.push_back({x, y, r});
    }

    auto supplies = solve(requests);
    for (auto box : supplies) {
        cout << box.x1 << " " << box.y1 << " ";
        cout << box.x2 << " " << box.y2 << "\n";
    }

    return 0;
}
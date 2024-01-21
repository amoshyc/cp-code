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

    int area() const { return (x2 - x1 + 1) * (y2 - y1 + 1); }

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
    bool contain(double x, double y) const {
        auto bool_x = (x1 <= x + 0.5) && (x + 0.5 <= x2);
        auto bool_y = (y1 <= y + 0.5) && (y + 0.5 <= y2);
        return bool_x && bool_y;
    }
    bool intersect(const B &b) const {
        auto dx = min(x2, b.x2) - max(x1, b.x1);
        auto dy = min(y2, b.y2) - max(y1, b.y1);
        return (dx > 0) && (dy > 0);
    }
};

int evaluate(const vector<R> &requests, const vector<B> &boxes) {
    double score = 0;
    for (int i = 0; i < sz(requests); i++) {
        double x = requests[i].x;
        double y = requests[i].y;
        double r = requests[i].r;
        if (boxes[i].contain(x, y)) {
            double s = boxes[i].area();
            double mn = min(s, r);
            double mx = max(s, r);
            double p = 1 - pow((1 - mn / mx), 2);
            score += p * 1e9;
        }
    }
    score = score / sz(requests);
    return int(round(score));
}

bool has_overlap(const vector<B> &boxes, int idx) {
    for (int i = 0; i < sz(boxes); i++) {
        if (i == idx)
            continue;
        if (boxes[i].intersect(boxes[idx]))
            return true;
    }
    return false;
}

void shrink(const vector<R> &requests, vector<B> &boxes) {
    for (int i = 0; i < sz(boxes); i++) {
        auto [x1, y1, x2, y2] = boxes[i];
        auto w = (x2 - x1 + 1);
        auto h = (y2 - y1 + 1);
        auto [x, y, r] = requests[i];
        if (w * h > r) {
            // left-right
            auto new_w = r / h;
            if (x > (x1 + x2) / 2) {
                boxes[i].x1 = x2 - new_w + 1;
            } else {
                boxes[i].x2 = x1 + new_w + 1;
            }
            boxes[i].clip_();
            if (boxes[i].contain(x, y) && has_overlap(boxes, i) == false) {
                continue;
            } else {
                boxes[i] = {x1, y1, x2, y2};
            }

            // top-down
            auto new_h = r / w;
            if (y > (y1 + y2) / 2) {
                boxes[i].y1 = y2 - new_h + 1;
            } else {
                boxes[i].y2 = y1 + new_h + 1;
            }
            boxes[i].clip_();
            if (boxes[i].contain(x, y) && has_overlap(boxes, i) == false) {
                continue;
            } else {
                boxes[i] = {x1, y1, x2, y2};
            }
        }
    }
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
            prob[i] = double(requests[i].r) / double(boxes[i].area());
            if (prob[i] < 1.0) {
                prob[i] = 0.0;
            }
        }
        discrete_distribution<int> distribution(prob.begin(), prob.end());
        int idx = distribution(generator);

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

        if (t % 100 == 0) {
            shrink(requests, boxes);
        }
    }

    shrink(requests, boxes);

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

    int best_score = -1;
    vector<B> best_boxes;
    for (int k = 0; k < 50; k++) {
        auto boxes = solve(requests);
        auto score = evaluate(requests, boxes);
        if (score > best_score) {
            best_score = score;
            best_boxes = boxes;
        }
    }

    for (auto box : best_boxes) {
        cout << box.x1 << " " << box.y1 << " ";
        cout << box.x2 << " " << box.y2 << "\n";
    }

    return 0;
}
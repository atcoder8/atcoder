#include <bits/stdc++.h>
#include <atcoder/all>
using namespace std;
using namespace atcoder;
using ll = long long;
using ull = unsigned long long;
using pll = std::pair<long long, long long>;
template <typename T> bool chmin(T &a, const T& b) { if (a > b) { a = b; return true; } return false; }
template <typename T> bool chmax(T &a, const T& b) { if (a < b) { a = b; return true; } return false; }
const long long INF = 2e18;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout << fixed << setprecision(15);

    vector<ll> primes{4, 9, 5, 7, 11, 13, 17, 19, 23};

    vector<vector<ll>> groups;
    ll cnt = 0;

    for (auto&& prime : primes) {
        vector<ll> group;

        for (ll i = 0; i < prime; i++) {
            group.push_back(cnt + 1);
            ++cnt;
        }

        groups.push_back(group);
    }

    ll M = cnt;

    cout << M << "\n";
    for (auto&& group : groups) {
        for (size_t i = 0; i < group.size(); i++) {
            cout << group[(i + 1) % group.size()] << " ";
        }
    }
    cout << endl;

    vector<ll> B(M);
    for (int i = 0; i < M; i++) cin >> B[i];

    vector<ll> rems;
    ll start_idx = 0;

    for (size_t i = 0; i < primes.size(); i++) {
        for (ll j = 0; j < M; j++) {
            if (groups[i][j] == B[start_idx]) {
                rems.push_back(j);
                break;
            }
        }

        start_idx += groups[i].size();
    }

    auto N = crt(rems, primes).first;
    cout << N << endl;
}

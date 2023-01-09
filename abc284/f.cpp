#include <bits/stdc++.h>
#include <atcoder/all>
using namespace std;
using namespace atcoder;
using ll = long long;
using ull = unsigned long long;
using pll = std::pair<long long, long long>;
template <typename T> bool chmin(T& a, const T& b) { return a > b && (a = b, true); }
template <typename T> bool chmax(T& a, const T& b) { return a < b && (a = b, true); }
const long long INF = 2e18;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout << fixed << setprecision(15);

    ll N;
    string T;
    cin >> N >> T;

    string A = T.substr(0, N), B = T.substr(N, 2 + N);
    reverse(B.begin(), B.end());

    auto X = A + B, Y = B + A;
    auto ZX = z_algorithm(X);
    ZX.push_back(0);
    auto ZY = z_algorithm(Y);
    ZY.push_back(0);

    for (ll i = 0; i <= N; i++) {
        if (ZX[2 * N - i] == i && ZY[N + i] == N - i) {
            cout << T.substr(0, i) << T.substr(N + i, 2 * N) << "\n";
            cout << i << "\n";
            exit(0);
        }
    }

    cout << -1 << "\n";
}

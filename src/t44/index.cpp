#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
#include <queue>
#include <numeric>

using namespace std;
typedef long long LL;
class Solution
{
#define rep(i, a, b) for (int i = (a); i <= (b); ++i)
#define re_(i, a, b) for (int i = (a); i < (b); ++i)
#define dwn(i, a, b) for (int i = (a); i >= (b); --i)
public:
  static const int N = (1 << 15) + 10;
  int dp[N];
  bool can[N];
  bool makesquare(vector<int> &a)
  {
    int n = a.size();
    LL goal = accumulate(a.begin(), a.end(), 0LL);
    if (goal % 4)
      return false;
    goal >>= 2;
    memset(dp, 0, sizeof dp);
    memset(can, 0, sizeof can);
    vector<int> legal;
    re_(S, 0, 1 << n)
    {
      LL u = 0;
      re_(i, 0, n)
      {
        if (!(S >> i & 1))
          continue;
        u += a[i];
      }
      if (u == goal)
        legal.push_back(S), can[S] = true;
    }
    re_(S, 0, 1 << n)
    {
      if ((1 << __builtin_popcount(S)) <= legal.size())
      {
        for (int S0 = S; S0; S0 = (S0 - 1) & S)
          if (can[S0])
            dp[S] = max(dp[S], dp[S ^ S0] + 1);
      }
      else
      {
        for (int &S0 : legal)
          if ((S & S0) == S0)
            dp[S] = max(dp[S], dp[S ^ S0] + 1);
      }
    }
    return dp[(1 << n) - 1] == 4;
  }
};

int main()
{
  Solution s;
  vector<int> v1{1, 1, 2, 2, 2};
  vector<int> v2{3, 3, 3, 3, 4};

  bool res1 = s.makesquare(v1);
  bool res2 = s.makesquare(v2);
}
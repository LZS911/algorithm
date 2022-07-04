#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
#include <queue>
#include <numeric>

using namespace std;
class Solution
{
public:
  vector<vector<int>> minimumAbsDifference(vector<int> &arr)
  {
    int n = arr.size();
    sort(arr.begin(), arr.end());

    int best = INT_MAX;
    vector<vector<int>> ans;
    for (int i = 0; i < n - 1; ++i)
    {
      int delta = arr[i + 1] - arr[i];
      if (delta < best)
      {
        best = delta;
        ans = {{arr[i], arr[i + 1]}};
      }
      else if (delta == best)
      {
        ans.emplace_back(initializer_list<int>{arr[i], arr[i + 1]});
      }
    }

    return ans;
  }
};

int main()
{
  Solution s;
  vector<int> req1 = {4, 2, 1, 3};
  vector<vector<int>> res1 = s.minimumAbsDifference(req1);
  vector<int> req2 = {1, 3, 6, 10, 15};
  vector<vector<int>> res2 = s.minimumAbsDifference(req2);
  vector<int> req3 = {3, 8, -10, 23, 19, -4, -14, 27};
  vector<vector<int>> res3 = s.minimumAbsDifference(req3);
}
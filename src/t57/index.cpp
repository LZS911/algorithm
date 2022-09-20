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
  bool canPartitionKSubsets(vector<int> &nums, int k)
  {
    int s = accumulate(nums.begin(), nums.end(), 0);
    if (s % k)
    {
      return false;
    }
    s /= k;
    int n = nums.size();
    vector<int> cur(k);
    function<bool(int)> dfs;
    dfs = [&](int i)
    {
      if (i == n)
      {
        return true;
      }
      for (int j = 0; j < k; ++j)
      {
        if (j && cur[j] == cur[j - 1])
        {
          continue;
        }
        cur[j] += nums[i];
        if (cur[j] <= s && dfs(i + 1))
        {
          return true;
        }
        cur[j] -= nums[i];
      }
      return false;
    };
    sort(nums.begin(), nums.end(), greater<int>());
    return dfs(0);
  }
};

int main()
{
  Solution s;

  vector<int> p1{4, 3, 2, 3, 5, 2, 1};
  bool res1 = s.canPartitionKSubsets(p1, 4);

  vector<int> p2{1, 2, 3, 4};
  bool res2 = s.canPartitionKSubsets(p2, 3);
}
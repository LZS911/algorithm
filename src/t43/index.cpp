#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
#include <queue>

using namespace std;
class Solution
{
public:
  int minMoves2(vector<int> &nums)
  {
    sort(nums.begin(), nums.end());
    int n = nums.size();
    long ans = 0x7fffffff, left = 0, right = 0, pre = 0;
    for (int cur : nums)
      right += cur;
    for (int i = 0, j = n; i < n; ++i, --j)
    {
      left += (nums[i] - pre) * i;
      right -= (nums[i] - pre) * j;
      ans = min(ans, left + right);
      pre = nums[i];
    }
    return ans;
  }
};

int main()
{
  Solution s;
  vector<int> v1{1, 2, 3};
  vector<int> v2{1, 10, 2, 9};
  int res1 = s.minMoves2(v1);
  int res2 = s.minMoves2(v2);
}
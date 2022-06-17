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
  int findPairs(vector<int> &nums, int k)
  {
    sort(nums.begin(), nums.end());
    int n = nums.size(), res = 0;
    for (int slow = 0, fast = 0; slow < n; ++slow)
    {
      if (slow != 0 && nums[slow] == nums[slow - 1])
        continue;
      fast = slow + 1;
      while (fast < n && nums[fast] - nums[slow] < k)
        ++fast;
      if (fast < n && nums[fast] - nums[slow] == k)
        ++res;
    }
    return res;
  }
};

int main()
{
  Solution s;
  vector<int> v1{3, 1, 4, 1, 5};
  vector<int> v2{1, 2, 3, 4, 5};
  vector<int> v3{1, 3, 1, 5, 4};

  int r1 = s.findPairs(v1, 2);
  int r2 = s.findPairs(v2, 1);
  int r3 = s.findPairs(v3, 0);
}
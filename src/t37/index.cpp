#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
using namespace std;

class Solution
{
public:
  int numSubarrayProductLessThanK(vector<int> &nums, int k)
  {
    if (k < 2)
      return 0;
    int n = nums.size(), l = 0, r = 0, ans = 0, prefix = 1;
    while (r != n)
    {
      prefix *= nums[r];
      while (prefix >= k)
      {
        prefix /= nums[l];
        ++l;
      }
      ++r;
      ans += r - l;
    }
    return ans;
  }
};

int main()
{
  Solution s;
  vector<int> v1{10, 5, 2, 6};
  vector<int> v2{1, 2, 3};
  int res1 = s.numSubarrayProductLessThanK(v1, 100);
  int res2 = s.numSubarrayProductLessThanK(v2, 0);
}
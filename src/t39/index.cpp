#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
#include <queue>

using namespace std;

class Solution
{
public:
  vector<int> findDuplicates(vector<int> &nums)
  {
    int n = nums.size();
    for (int i = 0; i < n; i++)
    {
      while (nums[i] != nums[nums[i] - 1])
      {
        swap(nums[i], nums[nums[i] - 1]);
      }
    }

    vector<int> ans;
    for (int i = 0; i < n; i++)
    {
      if (nums[i] != i + 1)
      {
        ans.push_back(nums[i]);
      }
    }

    return ans;
  }
};

int main()
{
  Solution s;
  vector<int> v1{4, 3, 2, 7, 8, 2, 3, 1};
  vector<int> res1 = s.findDuplicates(v1);
}
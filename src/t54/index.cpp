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
  int specialArray(vector<int> &nums)
  {
    int l = 1, r = nums.size() + 1;
    while (l < r)
    {
      int mid = (l + r) / 2;
      int cnt = 0;
      for (int &num : nums)
      {
        if (num >= mid)
          cnt++;
      }
      if (cnt == mid)
        return mid;
      if (cnt < mid)
        r = mid;
      else
        l = mid + 1;
    }
    return -1;
  }
};

int main()
{
  Solution s;
  vector r({3, 5});
  int res = s.specialArray(r);
}
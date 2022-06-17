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
  void duplicateZeros(vector<int> &arr)
  {
    int len = arr.size(), j = len - 1, k = len - 1;
    for (int i = 0; i <= j; ++i)
    {
      if (arr[i] == 0)
      {
        if (i == j)
        {
          arr[k--] = 0;
          --j;
          break;
        }
        --j;
      }
    }
    while (j >= 0)
    {
      if (arr[j] == 0)
        arr[k--] = 0;
      arr[k--] = arr[j--];
    }
  }
};

int main()
{
  Solution s;
  vector<int> v1{1, 0, 2, 3, 0, 4, 5, 0};
  vector<int> v2{1, 2, 3};

  s.duplicateZeros(v1);
  s.duplicateZeros(v2);
}
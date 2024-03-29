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
  int maximumSwap(int num)
  {
    string charArray = to_string(num);
    int n = charArray.size();
    int maxIdx = n - 1;
    int idx1 = -1, idx2 = -1;
    for (int i = n - 1; i >= 0; i--)
    {
      if (charArray[i] > charArray[maxIdx])
      {
        maxIdx = i;
      }
      else if (charArray[i] < charArray[maxIdx])
      {
        idx1 = i;
        idx2 = maxIdx;
      }
    }
    if (idx1 >= 0)
    {
      swap(charArray[idx1], charArray[idx2]);
      return stoi(charArray);
    }
    else
    {
      return num;
    }
  }
};

int main()
{
  Solution s;
  int res1 = s.maximumSwap(2736);
  int res2 = s.maximumSwap(9973);
}
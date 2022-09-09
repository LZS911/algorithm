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
  int minOperations(vector<string> &logs)
  {
    int res = 0;
    for (int i = 0; i < logs.size(); ++i)
    {
      if (logs[i] == "../")
      {
        res == 0 ? 0 : res--;
      }
      else if (logs[i] == "./")
      {
        continue;
      }
      else
      {
        res += 1;
      }
    }
    return res;
  }
};

int main()
{
  Solution s;
  vector<string> p1{"d1/", "d2/", "../", "d21/", "./"};
  int res1 = s.minOperations(p1);
}
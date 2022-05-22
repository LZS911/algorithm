#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
#include <queue>

using namespace std;

class Solution
{
private:
  int minNum = INT_MAX, num = 0;
  unordered_set<string> set;

public:
  int minMutation(string start, string end, vector<string> &bank)
  {
    if (start == end)
    {
      minNum = min(num, minNum);
      return minNum;
    }
    for (string &str : bank)
    {
      int diff = 0;
      for (int i = 0; i < str.size(); i++)
        if (str[i] != start[i])
          diff++;
      if (diff == 1 && set.find(str) == set.end())
      {
        set.insert(str);
        num += 1;
        minMutation(str, end, bank);
        num -= 1;
        set.erase(str);
      }
    }
    return minNum == INT_MAX ? -1 : minNum;
  }
};

int main()
{
  Solution s;
  vector<string> v1{"AACCGGTA"};
  int res1 = s.minMutation("AACCGGTT", "AACCGGTA", v1);
}
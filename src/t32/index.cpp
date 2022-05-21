#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
using namespace std;

class Solution
{
public:
  vector<int> shortestToChar(string s, char c)
  {
    unordered_set<int> set1;
    vector<int> ans;
    for (int i = 0; i < s.size(); i++)
    {
      if (s[i] == c)
        set1.insert(i);
    }

    for (int i = 0; i < s.size(); i++)
    {
      if (set1.count(i) == 1)
        ans.push_back(0);
      else
      {
        int minNum = INT_MAX;
        for (int num : set1)
        {
          minNum = min(minNum, abs(num - i));
        }
        ans.push_back(minNum);
      }
    }
    return ans;
  }
};
int main()
{
  Solution s;
  s.shortestToChar("loveleetcode", 'e');
}
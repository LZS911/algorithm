#include <iostream>
#include <vector>
#include <string>

using namespace std;

class Solution
{
public:
  vector<string> restoreIpAddresses(string s)
  {
    if (s.length() < 4 || s.length() > 12)
    {
      return vector<string>{};
    }
    res = {};

    dfs(s, vector<string>{}, 0);

    return res;
  }

private:
  vector<string> res;
  string join(vector<string> arr, string s)
  {
    string r = "";

    for (int i = 0; i < arr.size(); ++i)
    {
      r += arr[i] + (i == arr.size() - 1 ? "" : s);
    }
    return r;
  }
  void dfs(const string s, vector<string> path, int startIndex)
  {
    int length = s.length();

    if (path.size() == 4)
    {
      if (startIndex == length)
      {
        res.push_back(join(path, "."));
      }
      return;
    }

    if (startIndex == length)
    {
      return;
    }

    if (s[startIndex] == '0')
    {
      path.push_back("0");
      dfs(s, path, startIndex + 1);
      return;
    }

    for (int i = startIndex + 1; i <= length && i <= startIndex + 3; i++)
    {
      string v = s.substr(startIndex, i - startIndex);
      if (stoi(v) > 255 || stoi(v) < 0)
      {
        continue;
      }
      path.push_back(v);
      dfs(s, path, i);
      path.pop_back();
    }
  }
};

int main()
{
  Solution s;
  vector<string> res1 = s.restoreIpAddresses("25525511135");
  vector<string> res2 = s.restoreIpAddresses("0000");
  vector<string> res3 = s.restoreIpAddresses("101023");
}
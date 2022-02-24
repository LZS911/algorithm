#include <iostream>
#include <vector>
#include <string>
using namespace std;

class Solution
{
public:
  vector<vector<string>> partition(string s)
  {
    res = {};
    if (s.length() == 1)
    {
      res.push_back(vector<string>{s});
      return res;
    }
    dfs(s, 1, s, vector<string>{});
    return res;
  }

private:
  vector<vector<string>> res;
  bool isPalindrome(string source)
  {
    int length = source.length();
    if (length == 0)
    {
      return false;
    }
    if (length == 1)
    {
      return true;
    }
    bool flag = true;
    string left = source.substr(0, length / 2);
    string right = source.substr(length % 2 == 0 ? length / 2 : length / 2 + 1, length / 2);
    for (int i = 0; i < length / 2; ++i)
    {
      if (left[i] != right[length / 2 - i - 1])
      {
        flag = false;
      }
    }

    return flag;
  }
  void dfs(string s, int subLength, string origin, vector<string> path)
  {
    int length = s.length();

    if (length == 0)
    {
      res.push_back(path);
      return;
    }

     for (int i = 1; i <= length; ++i)
    {
      string left = s.substr(0, i);
      string right = s.substr(i, length - i);
      if (isPalindrome(left))
      {
        path.push_back(left);
        dfs(right, i + 1, origin, path);
        path.pop_back();
      }
    }
  }
};

int main()
{
  Solution s;
  vector<vector<string>> res1 = s.partition("aabb");
}
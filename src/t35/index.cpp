#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
using namespace std;

class Solution
{
public:
  static bool cmp(string &log1, string &log2)
  {
    int index1, index2;
    index1 = log1.find(' ');
    index2 = log2.find(' ');
    if (log1.substr(index1 + 1) != log2.substr(index2 + 1))
      return log1.substr(index1 + 1) < log2.substr(index2 + 1);
    return log1 < log2;
  }
  vector<string> reorderLogFiles(vector<string> &logs)
  {
    if (logs.size() <= 1)
      return logs;
    vector<string> res, tmp;
    for (int i = 0; i < logs.size(); i++)
    {
      int index = logs[i].find(' ');
      if (logs[i][index + 1] >= '0' && logs[i][index + 1] <= '9')
        tmp.push_back(logs[i]);
      else
        res.push_back(logs[i]);
    }
    sort(res.begin(), res.end(), cmp);
    res.insert(res.end(), tmp.begin(), tmp.end());
    return res;
  }
};

int main()
{
  Solution s;
}
#include <string>
#include <iostream>
#include <vector>
#include <unordered_map>

using namespace std;

class Solution
{
public:
  string mostCommonWord(string s, vector<string> &banned)
  {
    unordered_map<string, int> hash;
    string word;
    for (int i = 0; i < s.size(); i++)
    {
      if (s[i] == ' ' || s[i] == ',' || s[i] == '.')
      { // end
        if (!word.empty())
        {
          hash[word]++;
          word.clear();
        }
      }
      else if ((s[i] >= 'A' && s[i] <= 'Z') || (s[i] >= 'a' && s[i] <= 'z'))
      {
        if (s[i] >= 'A' && s[i] <= 'Z')
          s[i] += 32;
        word += s[i];
      }
    }

    if (!word.empty())
    {
      hash[word]++;
      word.clear();
    }

    for (auto x : banned)
    {
      hash.erase(x);
    }

    string res;
    for (auto &[k, v] : hash)
    {
      if (hash[res] < v)
      {
        res = k;
      }
    }
    return res;
  }
};

int main()
{
  Solution s;
  vector<string> v{"hit"};
  s.mostCommonWord("Bob hit a ball, the hit BALL flew far after it was hit.", v);
}
#include <string>
#include <iostream>
#include <vector>
#include <unordered_set>

using namespace std;

const static string MORSE[] = {
    ".-", "-...", "-.-.", "-..", ".", "..-.", "--.",
    "....", "..", ".---", "-.-", ".-..", "--", "-.",
    "---", ".--.", "--.-", ".-.", "...", "-", "..-",
    "...-", ".--", "-..-", "-.--", "--.."};

class Solution
{
public:
  int uniqueMorseRepresentations(vector<string> &words)
  {
    unordered_set<string> s;
    for (auto &word : words)
    {
      string str;
      for (auto &c : word)
      {
        str.append(MORSE[c - 'a']);
      }
      s.emplace(str);
    }
    return s.size();
  }
};

int main()
{
  Solution s;
  vector<string> v{"cab"};
  s.uniqueMorseRepresentations(v);
}
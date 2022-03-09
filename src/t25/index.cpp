#include <iostream>
#include <vector>
#include <string>
#include <cmath>
using namespace std;

class Solution
{
public:
  vector<int> platesBetweenCandles(string s, vector<vector<int>> &queries)
  {
    vector<int> res{};
    int sum1 = 0;
    int sum2 = 0;
    vector<int> preSum(s.length());
    for (int i = 0; i < s.length(); ++i)
    {
      if (s[i] == '*')
      {
        sum1 += 1;
      }
      else if (s[i] == '|')
      {
        sum2 += 1;
      }
      preSum[i] = sum1;
    }
    for (int i = 0; i < queries.size(); ++i)
    {
      // res.push_back(getBetweenCandlesCount(queries[i][0], queries[i][1], s));
    }
    return res;
  }

  // private:
  //   int getBetweenCandlesCount(int start, int end, string arr)
  //   {
  //     int startIndex = 0;
  //     int endIndex = 0;
  //     int res = 0;
  //     bool isFirst = true;
  //     for (int i = start; i <= end; ++i)
  //     {
  //       if (arr[i] == '|')
  //       {
  //         if (isFirst)
  //         {
  //           startIndex = i;
  //           isFirst = false;
  //         }
  //         else
  //         {
  //           endIndex = i;
  //           res += endIndex - startIndex - 1;
  //           startIndex = i;
  //         }
  //       }
  //     }
  //     return res;
  //   }
};
// class Solution
// {
// public:
//   vector<int> platesBetweenCandles(string s, vector<vector<int>> &queries)
//   {
//     int n = s.length();
//     vector<int> preSum(n);
//     for (int i = 0, sum = 0; i < n; i++)
//     {
//       if (s[i] == '*')
//       {
//         sum++;
//       }
//       preSum[i] = sum;
//     }
//     vector<int> left(n);
//     for (int i = 0, l = -1; i < n; i++)
//     {
//       if (s[i] == '|')
//       {
//         l = i;
//       }
//       left[i] = l;
//     }
//     vector<int> right(n);
//     for (int i = n - 1, r = -1; i >= 0; i--)
//     {
//       if (s[i] == '|')
//       {
//         r = i;
//       }
//       right[i] = r;
//     }
//     vector<int> ans;
//     for (auto &query : queries)
//     {
//       int x = right[query[0]], y = left[query[1]];
//       ans.push_back(x == -1 || y == -1 || x >= y ? 0 : preSum[y] - preSum[x]);
//     }
//     return ans;
//   }
// };

int main()
{
  Solution s;
  vector<vector<int>> n{vector<int>{1, 17}, vector<int>{4, 5}};
  s.platesBetweenCandles("***|**|*****|**||**|*", n);
}
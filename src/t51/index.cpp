#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
#include <queue>
#include <numeric>

using namespace std;
class MovingAverage
{
public:
  /** Initialize your data structure here. */
  queue<int> st;
  int n, cnt;
  MovingAverage(int size) : n(size), cnt(0)
  {
  }

  double next(int val)
  {
    if (st.size() == n)
    {
      cnt -= st.front(), st.pop();
    }
    cnt += val;
    st.push(val);
    return (double)cnt / st.size();
  }
};

int main()
{
}
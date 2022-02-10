#include <iostream>
#include <vector>
#include <string>

using namespace std;

class Solution
{
public:
    vector<vector<int>> combine(int n, int k)
    {
        res = {};
        vector<int> p;
        dfs(p, 0, n, k);
        return res;
    }

private:
    vector<vector<int>> res;
    void dfs(vector<int> &_path, int index, int limit, int count)
    {
        if (_path.size() == count)
        {
            res.push_back(_path);
            return;
        }

        for (int i = index + 1; i <= limit; ++i)
        {
            _path.push_back(i);
            dfs(_path, i, limit, count);
            _path.pop_back();
        }
    }
};

int main()
{
    Solution s1;
    vector<vector<int>> res1 = s1.combine(1, 1);
    vector<vector<int>> res2 = s1.combine(4, 2);
}

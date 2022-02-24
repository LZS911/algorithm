#include <iostream>
#include <vector>
#include <string>

using namespace std;
struct TreeNode
{
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};
class Solution
{
public:
  vector<vector<int>> res;
  vector<vector<int>> pathSum(TreeNode *root, int targetSum)
  {
    res = vector<vector<int>>{};
    if (root == nullptr)
    {
      return res;
    }
    if (root->val == targetSum && root->left == nullptr && root->right == nullptr)
    {
      res.push_back(vector<int>{root->val});
      return res;
    }
    dfs(root, targetSum, vector<int>{});
    return res;
  }

private:
  void dfs(TreeNode *r, int target, vector<int> path)
  {
    if (r == nullptr)
    {
      return;
    }
    path.push_back(r->val);
    if (target - r->val == 0 && r->right == nullptr && r->left == nullptr)
    {
      res.push_back(path);
      return;
    }
    dfs(r->left, target - r->val, path);
    dfs(r->right, target - r->val, path);
  }
};

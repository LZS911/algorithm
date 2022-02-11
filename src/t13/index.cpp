#include <iostream>
#include <vector>
#include <string>

using namespace std;

class Solution
{
public:
  bool exist(vector<vector<char>> &board, string word)
  {
    int row = board[0].size();
    int col = board.size();
    vector<vector<int>> visited(col, vector<int>(row));
    for (int i = 0; i < col; ++i)
    {
      for (int j = 0; j < row; ++j)
      {
        if (dfs(board, word, 0, j, i, visited))
        {
          return true;
        }
      }
    }
    return false;
  }

private:
  vector<vector<int>> path{{-1, 0}, {1, 0}, {0, 1}, {0, -1}};
  bool dfs(vector<vector<char>> &board, string word, int wordIndex, int startX, int startY, vector<vector<int>> visited)
  {

    if (wordIndex == word.size() - 1)
    {
      return word[wordIndex] == board[startY][startX];
    };
    visited[startY][startX] = 1;
    if (word[wordIndex] == board[startY][startX])
    {
      for (int i = 0; i < 4; i++)
      {
        int newX = startX + path[i][0];
        int newY = startY + path[i][1];

        int row = board[0].size();
        int col = board.size();

        if (!(newX < 0 || newX >= row || newY < 0 || newY >= col) && visited[newY][newX] == 0)
        {
          if (dfs(board, word, wordIndex + 1, newX, newY, visited))
          {
            return true;
          }
        }
      }
      visited[startY][startX] = 0;
    }

    return false;
  };
};

int main()
{
  Solution s;

  // vector<char> item1{'A', 'B', 'C', 'E'};
  // vector<char> item2{'S', 'F', 'C', 'S'};
  // vector<char> item3{'A', 'D', 'E', 'E'};
  // vector<vector<char>> board{item1, item2, item3};

  // bool res1 = s.exist(board, "ABCCED");
  // bool res2 = s.exist(board, "SEE");
  // bool res3 = s.exist(board, "ABCB");

  // vector<char> item4{'C', 'A', 'A'};
  // vector<char> item5{'A', 'A', 'A'};
  // vector<char> item6{'B', 'C', 'D'};
  // vector<vector<char>> board2{item4, item5, item6};
  // bool res4 = s.exist(board2, "AAB");

  vector<vector<char>> board3{
      {'a', 'a', 'a', 'a'}, {'a', 'a', 'a', 'a'}, {'a', 'a', 'a', 'a'}};

  bool res4 = s.exist(board3, "aaaaaaaaaaaaa");
}

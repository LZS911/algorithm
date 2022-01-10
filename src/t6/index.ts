export function generateParenthesis(n: number): string[] {
  const res: string[] = [];

  const backtrack = (lCount: number, rCount: number, brackets: string) => {
    if (lCount === 0 && rCount === 0) {
      res.push(brackets);
      return;
    }
    lCount > 0 && backtrack(lCount - 1, rCount, brackets + '(');
    lCount < rCount && backtrack(lCount, rCount - 1, brackets + ')');
  };

  backtrack(n, n, '');
  return res;
}

generateParenthesis(2);
generateParenthesis(3);

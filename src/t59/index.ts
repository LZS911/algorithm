export function scoreOfParentheses(s: string): number {
  const stk: number[] = [0];
  for (const c of s) {
    if (c == '(') stk.push(0);
    else {
      const cur = stk.pop()!;
      stk.push(stk.pop()! + Math.max(cur * 2, 1));
    }
  }
  return stk.pop()!;
}

const res1 = scoreOfParentheses('()');
const res2 = scoreOfParentheses('(())');
const res3 = scoreOfParentheses('()()');
const res4 = scoreOfParentheses('(()(()))');

console.log(res1, res2, res3, res4);

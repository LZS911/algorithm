export function halvesAreAlike(s: string): boolean {
  let left = 0,
    right = 0;
  const target = 'aeiouAEIOU';
  for (let i = 0; i < s.length / 2; ++i) {
    if (target.indexOf(s[i]) >= 0) {
      left++;
    }
    if (target.indexOf(s[s.length - 1 - i]) >= 0) {
      right++;
    }
  }

  return left === right;
}

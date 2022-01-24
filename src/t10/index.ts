export function permuteUnique(nums: number[]): number[][] {
  if (nums.length === 0) {
    return [];
  }
  const numsState = [...nums];
  numsState.sort((a, b) => a - b);
  const res: number[][] = [];

  const fn = (path: number[], used = new Map<number, boolean>()) => {
    if (path.length === nums.length) {
      res.push([...path]);
      return;
    }

    for (let i = 0; i < numsState.length; ++i) {
      if (used.get(i)) {
        continue;
      }
      if (i > 0 && numsState[i - 1] === numsState[i] && used.get(i - 1)) {
        continue;
      }
      path.push(numsState[i]);
      used.set(i, true);
      fn(path, used);
      path.pop();
      used.set(i, false);
    }
  };

  fn([]);
  return res;
}

permuteUnique([1, 1, 2]);

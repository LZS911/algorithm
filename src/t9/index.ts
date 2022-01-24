export function permute(nums: number[]): number[][] {
  if (nums.length === 0) {
    return [];
  }
  const res: number[][] = [];

  const fn = (path: number[]) => {
    if (path.length === nums.length) {
      res.push([...path]);
      return;
    }

    for (let i = 0; i < nums.length; ++i) {
      if (path.includes(nums[i])) {
        continue;
      }
      path.push(nums[i]);
      fn(path);
      path.pop();
    }
  };

  fn([]);

  return res;
}

permute([1, 2, 3]);

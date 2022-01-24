export function combinationSum2(candidates: number[], target: number): number[][] {
  const res: number[][] = [];
  const candidatesState = [...candidates];
  candidatesState.sort((a, b) => a - b);
  const fn = (sum: number, path: number[], index: number) => {
    if (sum === target) {
      res.push([...path]);
      return;
    }

    for (let i = index; i < candidatesState.length; ++i) {
      const val = candidatesState[i];
      if (val > target - sum) {
        continue;
      }
      if (i > index && candidatesState[i - 1] === candidatesState[i]) {
        continue;
      }

      path.push(val);
      fn(sum + val, path, i + 1);
      path.pop();
    }
  };
  fn(0, [], 0);
  return res;
}

combinationSum2([10, 1, 2, 7, 6, 1, 5], 8);

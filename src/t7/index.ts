export function combinationSum(candidates: number[], target: number): number[][] {
  if (target <= 0) {
    return [];
  }
  const res: number[][] = [];

  const fn = (residue: number, index: number, path: number[] = []) => {
    //判断条件
    if (residue === 0) {
      res.push([...path]);
      return;
    }

    /**
     * let i = index ***
     * 不往回头找, 否则会造成重复情况
     */
    for (let i = index; i < candidates.length; ++i) {
      //当前项大于剩余值, 直接跳过
      if (candidates[i] > residue) {
        continue;
      }
      path.push(candidates[i]);
      fn(residue - candidates[i], i, path);

      /**
       * 走到这里表示当前栈要么已经走完并且成功了, 要么失败了, 移除当前层
       */
      path.pop();
    }
  };

  fn(target, 0);

  return res;
}

combinationSum([2, 3, 6, 7], 7);
